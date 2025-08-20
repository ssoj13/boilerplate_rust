// Configuration management - persistent storage for app settings
use serde::{Deserialize, Serialize};  // Derive macros for JSON serialization
use std::fs;  // File system operations
use std::path::PathBuf;  // Owned path type

/// Main configuration structure - serializes to JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Window configuration settings
    pub window: WindowConfig,
    
    /// Application-specific settings (extensible for future features)
    pub app: AppConfig,
}

/// Window size, position, and display settings
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowConfig {
    /// Window width in logical pixels
    pub width: u32,
    /// Window height in logical pixels  
    pub height: u32,
    /// Window X position (None = let OS decide)
    pub pos_x: Option<i32>,
    /// Window Y position (None = let OS decide)
    pub pos_y: Option<i32>,
    /// Whether window is maximized
    pub maximized: bool,
}

/// Application-specific configuration (extensible)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    /// Last opened file path
    pub last_file: Option<PathBuf>,
    /// Animation auto-play on startup
    pub auto_play: bool,
    /// Animation speed multiplier
    pub animation_speed: f32,
}

impl Default for Config {
    /// Create default configuration values
    fn default() -> Self {
        Self {
            window: WindowConfig {
                width: 1280,        // Default window size
                height: 720,
                pos_x: None,        // Let OS choose initial position
                pos_y: None,
                maximized: false,   // Start windowed
            },
            app: AppConfig {
                last_file: None,            // No file loaded initially
                auto_play: false,           // Start paused
                animation_speed: 1.0,       // Normal speed
            },
        }
    }
}

impl Config {
    /// Get the configuration file path using platform-appropriate directory
    /// Windows: %APPDATA%/egui_opengl_app/config.json
    /// Linux: ~/.config/egui_opengl_app/config.json  
    /// macOS: ~/Library/Application Support/egui_opengl_app/config.json
    pub fn config_path() -> Option<PathBuf> {
        dirs::config_dir()  // Get platform config directory
            .map(|dir| dir.join("egui_opengl_app").join("config.json"))
    }

    /// Load configuration from file, creating default if doesn't exist
    pub fn load() -> Self {
        // Try to get config file path
        let Some(config_path) = Self::config_path() else {
            eprintln!("Warning: Could not determine config directory, using defaults");
            return Self::default();
        };

        // Try to read existing config file
        match fs::read_to_string(&config_path) {
            Ok(contents) => {
                // Parse JSON content
                match serde_json::from_str::<Config>(&contents) {
                    Ok(config) => {
                        println!("Loaded config from: {}", config_path.display());
                        config
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to parse config file: {}", e);
                        eprintln!("Using default configuration");
                        Self::default()
                    }
                }
            }
            Err(_) => {
                // File doesn't exist or can't be read - create default
                println!("Config file not found, creating default at: {}", config_path.display());
                let config = Self::default();
                config.save();  // Save default config for next time
                config
            }
        }
    }

    /// Save current configuration to file
    pub fn save(&self) {
        let Some(config_path) = Self::config_path() else {
            eprintln!("Warning: Could not determine config directory, unable to save");
            return;
        };

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Warning: Failed to create config directory: {}", e);
                return;
            }
        }

        // Serialize config to pretty JSON
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                // Write to file
                if let Err(e) = fs::write(&config_path, json) {
                    eprintln!("Warning: Failed to save config: {}", e);
                } else {
                    println!("Saved config to: {}", config_path.display());
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to serialize config: {}", e);
            }
        }
    }

    /// Update window position from current window state
    pub fn update_window_pos(&mut self, x: i32, y: i32) {
        self.window.pos_x = Some(x);
        self.window.pos_y = Some(y);
    }

    /// Update window size from current window state  
    pub fn update_window_size(&mut self, width: u32, height: u32) {
        self.window.width = width;
        self.window.height = height;
    }

}