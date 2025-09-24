use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::process::Command;
use std::path::Path;

fn run_test_and_save_output(dir: &str, threads: &str, output: &str, test_name: &str) {
    let test_folder = dir.split('/').last().unwrap_or(test_name);
    
    let benchmarks_dir = Path::new("tests/benchmarks");
    if !benchmarks_dir.exists() {
        create_dir_all(benchmarks_dir).expect("Failed to create benchmarks directory");
    }
    
    let benchmark_file = format!("tests/benchmarks/{}_{}_{}.txt", test_folder, threads, "threads");
    
    let output_result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(dir)
        .arg(threads)
        .arg(output)
        .output()
        .expect("Failed to execute cargo run");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&benchmark_file)
        .expect("Failed to open benchmark file");
    
    writeln!(file, "-- {} --", test_name).expect("Failed to write to benchmark file");
    
    file.write_all(&output_result.stdout).expect("Failed to write to benchmark file");
    
    if !output_result.status.success() {
        println!("Command failed with status: {}", output_result.status);
        println!("stderr: {}", String::from_utf8_lossy(&output_result.stderr));
        println!("stdout: {}", String::from_utf8_lossy(&output_result.stdout));

        writeln!(file, "Command failed with status: {}", output_result.status).expect("Failed to write to benchmark file");
        writeln!(file, "stderr: {}", String::from_utf8_lossy(&output_result.stderr)).expect("Failed to write to benchmark file");
    }

    assert!(Path::new(output).exists(), "Output file was not created");
}

#[test]
fn test_small_csv_2_threads() {
    let dir = "tests/data/test_small_csv";
    let threads = "2";
    let output = "tests/data/test_small_csv/output.json";
    
    run_test_and_save_output(dir, threads, output, "Test Small csv");
}

#[test]
fn test_small_csv_4_threads() {
    let dir = "tests/data/test_small_csv";
    let threads = "4";
    let output = "tests/data/test_small_csv/output.json";
    
    run_test_and_save_output(dir, threads, output, "Test Small csv");
}

#[test]
fn test_segmented_small_csv_2_threads() {
    let dir = "tests/data/test_segmented_small_csv";
    let threads = "2";
    let output = "tests/data/test_segmented_small_csv/output.json";
    
    run_test_and_save_output(dir, threads, output, "Test Segmented Small csv");
}

#[test]
fn test_segmented_small_csv_4_threads() {
    let dir = "tests/data/test_segmented_small_csv";
    let threads = "4";
    let output = "tests/data/test_segmented_small_csv/output.json";
    
    run_test_and_save_output(dir, threads, output, "Test Segmented Small csv");
}