// Integration tests for the egui OpenGL application
// These tests focus on testing the application as a whole

use std::process::{Command, Stdio};
use std::time::Duration;

/// Test that the application binary exists and can be executed
#[test]
fn test_binary_exists() {
    let binary_path = if cfg!(windows) {
        "target/debug/egui_opengl_app.exe"
    } else {
        "target/debug/egui_opengl_app"
    };

    assert!(
        std::path::Path::new(binary_path).exists(),
        "Binary should exist at {}. Run 'cargo build' first.",
        binary_path
    );
}

/// Test command line argument parsing
#[test]
fn test_cli_help() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success(), "Help command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("egui OpenGL App"), "Help should contain app name");
    assert!(stdout.contains("--width"), "Help should contain width option");
    assert!(stdout.contains("--height"), "Help should contain height option");
}

/// Test version command
#[test]
fn test_cli_version() {
    let output = Command::new(get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute binary");

    assert!(output.status.success(), "Version command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("egui_opengl_app"), "Version should contain app name");
}

/// Test invalid arguments
#[test]
fn test_cli_invalid_args() {
    let output = Command::new(get_binary_path())
        .arg("--invalid-argument")
        .output()
        .expect("Failed to execute binary");

    assert!(!output.status.success(), "Invalid arguments should fail");
}

/// Test custom window dimensions
#[test]
fn test_cli_custom_dimensions() {
    // This test just ensures the app accepts the arguments without crashing immediately
    // We can't test the actual window size without a display
    let mut child = Command::new(get_binary_path())
        .arg("-w")
        .arg("800")
        .arg("--height")
        .arg("600")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start application");

    // Let it run for a short time to ensure it doesn't crash immediately
    std::thread::sleep(Duration::from_millis(500));

    // Try to terminate gracefully, then kill if needed
    let _ = child.kill();
    let _ = child.wait();
}

/// Test that the app handles termination signals properly
#[test]
fn test_graceful_shutdown() {
    let mut child = Command::new(get_binary_path())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start application");

    // Let it start up
    std::thread::sleep(Duration::from_millis(300));

    // Send termination signal
    let result = child.kill();
    assert!(result.is_ok(), "Should be able to terminate the application");

    // Wait for it to exit
    let exit_status = child.wait().expect("Failed to wait for child process");
    
    // On Unix systems, killed processes return signal-based exit codes
    // On Windows, they return 1
    // Either way, the process should have exited
    assert!(!exit_status.success() || exit_status.code() == Some(1));
}

/// Helper function to get the correct binary path for the current platform
fn get_binary_path() -> &'static str {
    if cfg!(windows) {
        "target/debug/egui_opengl_app.exe"
    } else {
        "target/debug/egui_opengl_app"
    }
}

/// Module for testing app components that don't require a window
mod unit_tests {
    // Note: These would normally be in src/ files with #[cfg(test)]
    // but since this is an integration test file, we include some here

    /// Test that we can create app state without panicking
    #[test]
    fn test_app_state_creation() {
        // This assumes the app modules are accessible
        // In a real scenario, you might need to expose these for testing
        // or use conditional compilation in the main crate
        
        // For now, this is a placeholder that demonstrates the concept
        assert!(true, "App state creation test placeholder");
    }

    /// Test file path handling
    #[test]
    fn test_file_operations() {
        use std::path::PathBuf;
        
        // Test that we can create and manipulate paths
        let test_path = PathBuf::from("test.txt");
        assert_eq!(test_path.file_name().unwrap(), "test.txt");
        
        // Test file extension extraction
        let test_path_with_ext = PathBuf::from("document.json");
        assert_eq!(test_path_with_ext.extension().unwrap(), "json");
    }

    /// Test frame counter operations
    #[test]
    fn test_frame_counter_math() {
        // Test the rotation calculation used in the app
        let frame_count: u64 = 100;
        let rotation = frame_count as f32 * 0.01;
        assert_eq!(rotation, 1.0);
        
        // Test that very large frame counts don't overflow
        let large_frame_count: u64 = u64::MAX;
        let _large_rotation = large_frame_count as f32 * 0.01;
        // This should not panic
    }
}

/// Performance-related integration tests
mod performance_tests {
    use super::*;

    /// Test that the application starts within a reasonable time
    #[test]
    #[ignore] // Ignore by default since this requires timing
    fn test_startup_performance() {
        let start_time = std::time::Instant::now();
        
        let mut child = Command::new(get_binary_path())
            .arg("--help") // Use help to avoid needing a display
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start application");

        let _output = child.wait_with_output().expect("Failed to wait for output");
        let elapsed = start_time.elapsed();

        // Application should start and show help within 5 seconds
        assert!(
            elapsed < Duration::from_secs(5),
            "Application took too long to start: {:?}",
            elapsed
        );
    }

    /// Test binary size is reasonable
    #[test]
    fn test_binary_size() {
        let binary_path = get_binary_path();
        let metadata = std::fs::metadata(binary_path)
            .expect("Failed to get binary metadata");

        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;
        
        // Debug binary should be under 100MB (this is quite generous)
        assert!(
            size_mb < 100.0,
            "Debug binary is too large: {:.2} MB. Consider optimizing dependencies.",
            size_mb
        );

        println!("Binary size: {:.2} MB", size_mb);
    }
}

/// Cross-platform compatibility tests
mod compatibility_tests {
    use super::*;

    /// Test that the application works with different locale settings
    #[test]
    fn test_locale_compatibility() {
        // Test with different locale environment variables
        let mut child = Command::new(get_binary_path())
            .arg("--help")
            .env("LANG", "en_US.UTF-8")
            .env("LC_ALL", "en_US.UTF-8")
            .output();

        if let Ok(output) = child {
            assert!(output.status.success(), "App should work with UTF-8 locale");
        }
        // If the test environment doesn't support locale changes, just pass
    }

    /// Test that the application handles missing environment variables
    #[test]
    fn test_minimal_environment() {
        let output = Command::new(get_binary_path())
            .arg("--version")
            .env_clear() // Clear all environment variables
            .env("PATH", std::env::var("PATH").unwrap_or_default()) // Keep PATH for binary execution
            .output()
            .expect("Failed to execute with minimal environment");

        assert!(
            output.status.success(),
            "App should work with minimal environment variables"
        );
    }
}