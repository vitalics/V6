use anyhow::Result;
use clap::Parser;

mod cli;
mod engine;

use cli::{
    display_test_config, init_command, parse_iterations_override, validate_file_exists, Cli,
    Commands,
};
use engine::{
    create_fresh_runtime, extract_duration, extract_iteration_function, extract_iterations,
    extract_timeout, extract_vus, run_load_test,
};

async fn run_command(
    file: &str,
    iterations_override: Option<String>,
    duration_override: Option<f64>,
    timeout_override: Option<f64>,
    vus_override: Option<usize>,
) -> Result<()> {
    validate_file_exists(file)?;

    let js_content = std::fs::read_to_string(file)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file, e))?;

    // Create fresh runtime for configuration extraction only
    let config_runtime = create_fresh_runtime()?;

    // Execute script in config runtime to extract configuration
    {
        let mut runtime = config_runtime.lock().unwrap();
        let _ = runtime.execute_script("<v6/config-extraction>", js_content.clone());
    }

    // Extract iterations, duration, timeout, and vus from globalThis
    let base_iterations = extract_iterations(config_runtime.clone())?;
    let base_duration = extract_duration(config_runtime.clone())?;
    let base_timeout = extract_timeout(config_runtime.clone())?;
    let base_vus = extract_vus(config_runtime.clone())?;
    let iteration_fn = extract_iteration_function(config_runtime.clone())?;

    // Apply CLI overrides
    let iterations = iterations_override
        .map(|s| parse_iterations_override(&s))
        .unwrap_or(base_iterations);
    let duration = duration_override.unwrap_or(base_duration);
    let iteration_timeout_secs = timeout_override.unwrap_or(base_timeout);
    let vus = vus_override.unwrap_or(base_vus);

    // Display configuration
    display_test_config(file, iterations, duration, iteration_timeout_secs, vus);

    // Run the load test
    run_load_test(
        iterations,
        duration,
        iteration_timeout_secs,
        vus,
        &iteration_fn,
    )
    .await?;

    // Clean up config runtime
    drop(config_runtime);

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {
            file,
            iterations,
            duration,
            timeout,
            vus,
        } => init_command(file, iterations, *duration, *timeout, *vus),
        Commands::Run {
            file,
            iterations,
            duration,
            timeout,
            vus,
        } => run_command(file, iterations.clone(), *duration, *timeout, *vus).await,
    }
}
