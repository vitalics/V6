use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Parser)]
#[command(name = "v6")]
#[command(about = "A load testing tool with JavaScript runtime", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new test file
    Init {
        /// Name of the test file to create
        #[arg(short, long)]
        file: String,
        
        /// Number of iterations (default: 1, use 'inf' for infinite)
        #[arg(short, long, default_value = "1")]
        iterations: String,
        
        /// Duration in seconds (default: 10)
        #[arg(short, long, default_value = "10")]
        duration: f64,
        
        /// Timeout per iteration in seconds (default: 30)
        #[arg(short, long, default_value = "30")]
        timeout: f64,
        
        /// Number of virtual users (default: 1)
        #[arg(short, long, default_value = "1")]
        vus: usize,
    },
    /// Run a test file
    Run {
        /// Path to the test file
        file: String,
        
        /// Number of iterations (overrides file config)
        #[arg(short, long)]
        iterations: Option<String>,
        
        /// Duration in seconds (overrides file config)
        #[arg(short, long)]
        duration: Option<f64>,
        
        /// Timeout per iteration in seconds (overrides file config)
        #[arg(short, long)]
        timeout: Option<f64>,
        
        /// Number of virtual users (overrides file config)
        #[arg(short, long)]
        vus: Option<usize>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct TestConfig {
    pub iterations: String,
    pub duration: f64,
    pub timeout: f64,
    pub vus: usize,
}

pub fn init_command(file: &str, iterations: &str, duration: f64, timeout: f64, vus: usize) -> Result<()> {
    let template = format!(r#"console.log("Starting test: {file}");

defineConfig({{
  iterations: {iterations_value},
  duration: {duration},
  timeout: {timeout}, // max timeout for each iteration
  vus: {vus}, // Virtual Users
  iteration: async function () {{
    console.log("[JS] iteration starting");
    
    // Add your test logic here
    // Example: await setTimeout(1000);
    
    console.log("[JS] iteration completed");
  }},
}});
"#, 
        file = file,
        iterations_value = if iterations == "inf" { "Infinity".to_string() } else { iterations.to_string() },
        duration = duration,
        timeout = timeout,
        vus = vus
    );
    
    std::fs::write(file, template)?;
    println!("✅ Created test file: {}", file);
    println!("📝 Edit the file to add your test logic inside the iteration function");
    Ok(())
}

pub fn parse_iterations_override(iterations_str: &str) -> f64 {
    match iterations_str {
        "inf" | "infinity" => f64::INFINITY,
        _ => iterations_str.parse().unwrap_or(1.0),
    }
}

pub fn validate_file_exists(file: &str) -> Result<()> {
    if !Path::new(file).exists() {
        eprintln!("❌ File not found: {}", file);
        std::process::exit(1);
    }
    Ok(())
}

pub fn display_test_config(
    file: &str,
    iterations: f64,
    duration: f64,
    timeout: f64,
    vus: usize,
) {
    println!("🚀 Starting load test");
    println!("📁 File: {}", file);
    if iterations.is_infinite() {
        println!("🔄 Iterations: ∞ (infinite)");
    } else {
        println!("🔄 Iterations: {}", iterations as u64);
    }
    println!("⏱️  Duration: {}s", duration);
    println!("⏰ Timeout per iteration: {}s", timeout);
    println!("👥 Virtual Users: {}", vus);
    println!("{}", "─".repeat(50));
}