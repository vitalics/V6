use anyhow::Result;
use deno_core::{extension, op2};
use deno_error::JsErrorBox;
use std::{
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};
use tokio::time::timeout;

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/V6_SNAPSHOT.bin"));

pub struct TsModuleLoader;

impl deno_core::ModuleLoader for TsModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        _referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<deno_core::ModuleSpecifier, deno_core::error::ModuleLoaderError> {
        deno_core::resolve_url(specifier).map_err(Into::into)
    }

    fn load(
        &self,
        _module_specifier: &deno_core::ModuleSpecifier,
        _maybe_referrer: Option<&deno_core::ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        // Simple implementation for basic functionality
        todo!("Module loading not implemented")
    }
}

extension!(v6, ops = [op_set_timeout, op_fetch],
    esm_entry_point = "ext:v6/runtime.js",
    esm = [dir "src", "runtime.js"],);

#[op2(async, stack_trace)]
async fn op_set_timeout(delay: f64) {
    tokio::time::sleep(std::time::Duration::from_millis(delay as u64)).await;
}

// Global HTTP client for reuse
static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[op2(async, stack_trace)]
#[serde]
async fn op_fetch(
    #[string] url: String,
    #[serde] options: Option<serde_json::Value>,
) -> Result<serde_json::Value, JsErrorBox> {
    let client = HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(std::time::Duration::from_secs(30))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("V6/0.1.0")
            .connect_timeout(std::time::Duration::from_secs(10))
            .local_address(None)
            .build()
            .unwrap()
    });
    let mut request = client.get(&url);

    // Parse options if provided
    if let Some(opts) = options {
        // Set HTTP method
        if let Some(method) = opts.get("method").and_then(|v| v.as_str()) {
            request = match method.to_uppercase().as_str() {
                "GET" => client.get(&url),
                "POST" => client.post(&url),
                "PUT" => client.put(&url),
                "DELETE" => client.delete(&url),
                "PATCH" => client.patch(&url),
                "HEAD" => client.head(&url),
                _ => panic!("Unsupported HTTP method: {}", method),
            };
        }

        // Set headers
        if let Some(headers) = opts.get("headers").and_then(|v| v.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key, value_str);
                }
            }
        }

        // Set body
        if let Some(body) = opts.get("body").and_then(|v| v.as_str()) {
            request = request.body(body.to_string());
        }
    }

    let response_result = request.send().await;
    // let _request_duration = request_start.elapsed().as_nanos() as f64;

    match response_result {
        Ok(response) => {
            let status = response.status().as_u16();
            let is_success = (200..300).contains(&status);

            // Don't record stats here - let runtime.js handle it to avoid double recording

            let headers: std::collections::HashMap<String, String> = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect();

            let text = response
                .text()
                .await
                .map_err(|e| JsErrorBox::type_error(e.to_string()))?;

            Ok(serde_json::json!({
              "status": status,
              "ok": is_success,
              "statusText": reqwest::StatusCode::from_u16(status).unwrap().canonical_reason().unwrap_or(""),
              "headers": headers,
              "body": text
            }))
        }
        Err(e) => {
            // Don't record stats here - let runtime.js handle it to avoid double recording
            Err(JsErrorBox::type_error(e.to_string()))
        }
    }
}

pub fn extract_iterations(js_runtime: Arc<Mutex<deno_core::JsRuntime>>) -> Result<f64> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let iterations_script =
        deno_core::v8::String::new(&mut scope, "globalThis.currentConfig.iterations").unwrap();

    let compiled_code =
        deno_core::v8::Script::compile(&mut scope, iterations_script, None).unwrap();

    if let Some(result) = compiled_code.run(&mut scope) {
        if let Some(number) = result.number_value(&mut scope) {
            return Ok(number);
        }
    }

    Ok(1.0) // Default to 1 iteration if not found
}

pub fn extract_duration(js_runtime: Arc<Mutex<deno_core::JsRuntime>>) -> Result<f64> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let duration_script =
        deno_core::v8::String::new(&mut scope, "globalThis.currentConfig.duration").unwrap();

    let compiled_code = deno_core::v8::Script::compile(&mut scope, duration_script, None).unwrap();

    if let Some(result) = compiled_code.run(&mut scope) {
        if let Some(number) = result.number_value(&mut scope) {
            return Ok(number);
        }
    }

    Ok(10.0) // Default to 10 seconds if not found
}

pub fn extract_timeout(js_runtime: Arc<Mutex<deno_core::JsRuntime>>) -> Result<f64> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let timeout_script =
        deno_core::v8::String::new(&mut scope, "globalThis.currentConfig.timeout").unwrap();

    let compiled_code = deno_core::v8::Script::compile(&mut scope, timeout_script, None).unwrap();

    if let Some(result) = compiled_code.run(&mut scope) {
        if let Some(number) = result.number_value(&mut scope) {
            return Ok(number);
        }
    }

    Ok(30.0) // Default to 30 seconds if not found
}

pub fn extract_vus(js_runtime: Arc<Mutex<deno_core::JsRuntime>>) -> Result<usize> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let vus_script =
        deno_core::v8::String::new(&mut scope, "globalThis.currentConfig.vus").unwrap();

    let compiled_code = deno_core::v8::Script::compile(&mut scope, vus_script, None).unwrap();

    if let Some(result) = compiled_code.run(&mut scope) {
        if let Some(number) = result.number_value(&mut scope) {
            return Ok(number as usize);
        }
    }

    Ok(1) // Default to 1 VU if not found
}

pub fn create_fresh_runtime() -> Result<Arc<Mutex<deno_core::JsRuntime>>> {
    let js_runtime = Arc::new(Mutex::new(deno_core::JsRuntime::new(
        deno_core::RuntimeOptions {
            module_loader: Some(Rc::new(TsModuleLoader)),
            startup_snapshot: Some(RUNTIME_SNAPSHOT),
            extensions: vec![v6::init_ops_and_esm()],
            ..Default::default()
        },
    )));
    Ok(js_runtime)
}

pub fn extract_iteration_function(js_runtime: Arc<Mutex<deno_core::JsRuntime>>) -> Result<String> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let iteration_script =
        deno_core::v8::String::new(&mut scope, "globalThis.currentConfig.iteration.toString()")
            .unwrap();

    let compiled_code = deno_core::v8::Script::compile(&mut scope, iteration_script, None).unwrap();

    if let Some(result) = compiled_code.run(&mut scope) {
        if let Some(string_val) = result.to_string(&mut scope) {
            return Ok(string_val.to_rust_string_lossy(&mut scope));
        }
    }

    Ok("function() {}".to_string()) // Default empty function if not found
}

pub fn setup_runtime_config(
    js_runtime: Arc<Mutex<deno_core::JsRuntime>>,
    iterations: f64,
    duration: f64,
    timeout: f64,
    vus: usize,
    iteration_fn: &str,
) -> Result<()> {
    let mut runtime = js_runtime.lock().unwrap();
    let mut scope = runtime.handle_scope();

    let setup_script = format!(
        r#"
        globalThis.currentConfig = {{
            iterations: {},
            duration: {},
            timeout: {},
            vus: {},
            iteration: {}
        }};

        // Add global sleep function
        globalThis.sleep = function(ms) {{
            return Deno.core.opAsync("op_set_timeout", ms);
        }};
        "#,
        if iterations.is_infinite() {
            "Infinity".to_string()
        } else {
            iterations.to_string()
        },
        duration,
        timeout,
        vus,
        iteration_fn
    );

    let v8_string = deno_core::v8::String::new(&mut scope, &setup_script).unwrap();
    let compiled_code = deno_core::v8::Script::compile(&mut scope, v8_string, None).unwrap();
    compiled_code.run(&mut scope);

    Ok(())
}

pub async fn run_iteration(
    i: usize,
    vu_id: usize,
    js_runtime: Arc<Mutex<deno_core::JsRuntime>>,
    compiled_script: Arc<deno_core::v8::Global<deno_core::v8::Script>>,
    iteration_timeout: Duration,
) {
    let iteration_future = async {
        // Execute the script once
        {
            let mut runtime = js_runtime.lock().unwrap();
            let mut scope = runtime.handle_scope();
            let local_script = deno_core::v8::Local::new(&mut scope, &*compiled_script);
            local_script.run(&mut scope);
            drop(scope);
        }

        // Pump the event loop until completion
        loop {
            let poll_result = {
                let mut runtime = js_runtime.lock().unwrap();

                // Force memory cleanup much less frequently for better performance
                if i % 5000 == 0 {
                    let mut scope = runtime.handle_scope();
                    scope.low_memory_notification();
                }

                // Poll the event loop to handle async operations
                let waker = futures::task::noop_waker();
                let mut cx = std::task::Context::from_waker(&waker);
                let poll_options = deno_core::PollEventLoopOptions {
                    wait_for_inspector: false,
                    pump_v8_message_loop: true,
                };

                runtime.poll_event_loop(&mut cx, poll_options)
            };

            match poll_result {
                std::task::Poll::Ready(Ok(())) => break, // Completed
                std::task::Poll::Ready(Err(e)) => {
                    println!("Task {} (VU {}) failed: {:?}", i, vu_id, e);
                    break; // Break on error
                }
                std::task::Poll::Pending => {
                    // Wait a small amount for async operations to progress
                    tokio::time::sleep(Duration::from_millis(1)).await;
                    continue;
                }
            }
        }
    };

    match timeout(iteration_timeout, iteration_future).await {
        Ok(_) => {}
        Err(_) => println!(
            "Task {} (VU {}) timed out after {:?}",
            i, vu_id, iteration_timeout
        ),
    }
}

pub fn create_shared_runtime(
    iterations: f64,
    duration: f64,
    timeout: f64,
    vus: usize,
    iteration_fn: &str,
) -> Result<(
    Arc<Mutex<deno_core::JsRuntime>>,
    Arc<deno_core::v8::Global<deno_core::v8::Script>>,
)> {
    let runtime = create_fresh_runtime()?;
    setup_runtime_config(
        runtime.clone(),
        iterations,
        duration,
        timeout,
        vus,
        iteration_fn,
    )?;

    // Pre-compile the script for maximum performance
    let compiled_script = {
        let mut rt = runtime.lock().unwrap();
        let mut scope = rt.handle_scope();
        let script_source = "globalThis.currentConfig.iteration();";
        let v8_string = deno_core::v8::String::new(&mut scope, script_source).unwrap();
        let script = deno_core::v8::Script::compile(&mut scope, v8_string, None).unwrap();
        Arc::new(deno_core::v8::Global::new(&mut scope, script))
    };

    Ok((runtime, compiled_script))
}

pub async fn run_load_test(
    iterations: f64,
    duration: f64,
    iteration_timeout_secs: f64,
    vus: usize,
    iteration_fn: &str,
) -> Result<()> {
    let iteration_timeout = Duration::from_secs_f64(iteration_timeout_secs);

    let is_infinite = iterations.is_infinite();
    let max_iterations = if is_infinite {
        u64::MAX // Run until timeout
    } else {
        iterations as u64
    };

    let execution_duration = if is_infinite {
        Duration::from_secs_f64(duration)
    } else {
        Duration::from_secs(60) // Use 60s timeout for finite iterations as fallback
    };

    // Use LocalSet for task-local execution
    let local = tokio::task::LocalSet::new();

    let task_future = local.run_until(async move {
        // Prepare compiled script source once inside run_until block scope
        let _script_source = "globalThis.currentConfig.iteration();".to_string();

        let mut handles = Vec::new();
        // Limit concurrent tasks to prevent memory exhaustion
        let max_concurrent_tasks = (vus * 100).min(1000); // Limit to prevent memory issues
        let max_tasks = if is_infinite {
            max_concurrent_tasks
        } else {
            max_iterations.min(max_concurrent_tasks as u64) as usize
        };

        let start_time = Instant::now();
        let mut task_counter = 0usize;

        // Create single shared runtime and pre-compiled script
        let (shared_runtime, shared_script) = match create_shared_runtime(iterations, duration, iteration_timeout_secs, vus, &iteration_fn) {
            Ok((runtime, script)) => (runtime, script),
            Err(e) => {
                println!("Failed to create shared runtime: {}", e);
                return;
            }
        };

        let mut active_handles = Vec::new();

        if is_infinite {
            // For infinite iterations, continuously spawn tasks across VUs
            loop {
                if start_time.elapsed() >= execution_duration {
                    println!("{} second timeout reached for infinite iterations", duration);
                    break;
                }

                // Spawn tasks across all VUs using shared runtime
                for vu_id in 0..vus {
                    let runtime_clone = shared_runtime.clone();
                    let script_clone = shared_script.clone();
                    let timeout_clone = iteration_timeout;
                    let task_id = task_counter;
                    task_counter += 1;

                    let handle = tokio::task::spawn_local(async move {
                        run_iteration(task_id, vu_id, runtime_clone, script_clone, timeout_clone).await;
                    });
                    active_handles.push(handle);
                }

                // Clean up completed handles periodically
                active_handles.retain(|handle| !handle.is_finished());

                // Periodic memory cleanup for shared runtime - much less frequent
                if task_counter % 10000 == 0 {
                    if let Ok(mut runtime) = shared_runtime.try_lock() {
                        let mut scope = runtime.handle_scope();
                        scope.low_memory_notification();
                    }
                }

                // Small delay to prevent busy looping
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        } else {
            // For finite iterations, distribute tasks across VUs
            let tasks_per_vu = max_tasks / vus.max(1);
            let remaining_tasks = max_tasks % vus.max(1);

            for vu_id in 0..vus {
                let vu_task_count = if vu_id < remaining_tasks {
                    tasks_per_vu + 1
                } else {
                    tasks_per_vu
                };

                for task_id in 0..vu_task_count {
                    let global_task_id = vu_id * tasks_per_vu + task_id + (vu_id.min(remaining_tasks));

                    if global_task_id >= max_iterations as usize {
                        break;
                    }

                    let runtime_clone = shared_runtime.clone();
                    let script_clone = shared_script.clone();
                    let timeout_clone = iteration_timeout;
                    let handle = tokio::task::spawn_local(async move {
                        run_iteration(global_task_id, vu_id, runtime_clone, script_clone, timeout_clone).await;
                    });
                    handles.push(handle);
                }
            }
        }

        // Wait for all active tasks to complete before cleanup
        if is_infinite {
            println!("Waiting for {} active tasks to complete...", active_handles.len());
            for handle in active_handles {
                let _ = handle.await;
            }
            let rate = task_counter as f64 / execution_duration.as_secs_f64();
            println!(
                "Completed {} tasks across {} VUs (infinite iterations with {}s timeout) - Rate: {:.2} iterations/sec",
                task_counter, vus, duration, rate
            );
        } else {
            let completed_tasks = handles.len();
            for task in handles {
                let _ = task.await;
            }
            println!("All {} tasks completed across {} VUs", completed_tasks, vus);
        }

        // Final cleanup for shared runtime
        if let Ok(mut runtime) = shared_runtime.try_lock() {
            let mut scope = runtime.handle_scope();
            scope.low_memory_notification();
        }

        // Clean up shared resources
        drop(shared_script);
        drop(shared_runtime);
        });

    // Apply timeout for infinite iterations
    if is_infinite {
        match timeout(execution_duration, task_future).await {
            Ok(_) => println!("Tasks completed within timeout"),
            Err(_) => println!(
                "{} second timeout reached for infinite iterations",
                duration
            ),
        }
    } else {
        task_future.await;
    }

    Ok(())
}
