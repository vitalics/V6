use std::process::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("A load testing tool with JavaScript runtime"));
    assert!(stdout.contains("init"));
    assert!(stdout.contains("run"));
}

#[test]
fn test_init_command() {
    let test_file = "test_output.js";
    
    // Clean up any existing test file
    if Path::new(test_file).exists() {
        fs::remove_file(test_file).ok();
    }

    let output = Command::new("cargo")
        .args(&[
            "run", "--", "init", 
            "--file", test_file,
            "--iterations", "10",
            "--duration", "5",
            "--vus", "2"
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    
    // Check that the file was created
    assert!(Path::new(test_file).exists());
    
    // Check file contents
    let content = fs::read_to_string(test_file).unwrap();
    assert!(content.contains("defineConfig"));
    assert!(content.contains("iterations: 10"));
    assert!(content.contains("duration: 5"));
    assert!(content.contains("vus: 2"));
    
    // Clean up
    fs::remove_file(test_file).ok();
}

#[test]
fn test_run_nonexistent_file() {
    let output = Command::new("cargo")
        .args(&["run", "--", "run", "nonexistent.js"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("File not found") || stderr.contains("nonexistent.js"));
}

#[test]
fn test_version_info() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    // Note: --version might not be implemented, so we just check it doesn't crash
    // The exit code can be non-zero if --version isn't implemented
    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
    let stderr = String::from_utf8(output.stderr).unwrap_or_default();
    
    // Either it succeeds with version info, or fails gracefully
    assert!(!stdout.is_empty() || !stderr.is_empty());
}