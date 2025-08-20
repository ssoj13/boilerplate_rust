// Import PathBuf - a owned, growable file system path (like String but for paths)
use std::path::PathBuf;

/// Application state management - holds all our app's runtime data
pub struct AppState {
    // pub = public field, accessible from other modules
    pub file_open_dialog: bool,        // Flag to trigger file dialog
    pub playing: bool,                 // Is animation currently playing?
    pub mouse_pos: (f32, f32),         // Mouse position as tuple (x, y)
    pub status_text: String,           // Text to show in status bar (String = owned string)
    pub frame_count: u64,              // Current animation frame (u64 = unsigned 64-bit int)
    pub current_file: Option<PathBuf>, // Currently opened file (Option = maybe has a file)
}

// Implementation block - contains methods for AppState
impl AppState {
    // Associated function (no &self) - like a static method or constructor
    pub fn new() -> Self {  // Self = AppState (shorthand for the current type)
        Self {  // Self {} = struct literal, creates new instance
            file_open_dialog: false,                    // Start with no dialog open
            playing: false,                             // Start paused
            mouse_pos: (0.0, 0.0),                     // Mouse at origin
            status_text: String::from("Ready"),        // String::from = convert &str to String
            frame_count: 0,                            // Start at frame 0
            current_file: None,                        // No file loaded initially
        }
    }

    // Instance method (&mut self) - modifies the object
    pub fn toggle_play(&mut self) {  // &mut self = mutable borrow of self
        self.playing = !self.playing;  // ! = logical NOT operator
    }

    // Alias for toggle_play for consistency with keyboard shortcuts
    pub fn toggle_playing(&mut self) {
        self.toggle_play();
    }

    // Advance animation by one frame
    pub fn step(&mut self) {
        self.frame_count += 1;  // += is shorthand for self.frame_count = self.frame_count + 1
    }

    // Reset animation to beginning
    pub fn reset(&mut self) {
        self.frame_count = 0;      // Back to frame 0
        self.playing = false;      // And stop playing
    }

    // Update mouse position from UI events
    pub fn update_mouse(&mut self, x: f32, y: f32) {
        self.mouse_pos = (x, y);  // Store as tuple
    }

    // Build status string for display (&self = immutable borrow, doesn't change self)
    pub fn get_status(&self) -> String {
        // Build file info part of status
        let file_info = if let Some(path) = &self.current_file {  // if let = pattern match on Option
            format!(" | File: {}", 
                path.file_name()                    // Get filename part only
                    .unwrap_or_default()             // Use empty if no filename
                    .to_string_lossy()               // Convert OsStr to String (handles Unicode)
            )
        } else {
            String::new()  // Empty string if no file
        };
        
        // format! macro - like printf but type-safe!
        format!(
            "Mouse: ({:.1}, {:.1}) | Frame: {}{}{}",  // {:.1} = float with 1 decimal place
            self.mouse_pos.0,          // Access tuple element 0 (x)
            self.mouse_pos.1,          // Access tuple element 1 (y)
            self.frame_count, 
            file_info,
            // Conditional expression using if-else
            if !self.status_text.is_empty() { 
                format!(" | {}", self.status_text) 
            } else { 
                String::new() 
            }
        )
    }

    // Set the current file and update status
    pub fn set_current_file(&mut self, file_path: PathBuf) {
        self.current_file = Some(file_path.clone());  // clone() = create a copy of PathBuf
        self.status_text = format!("Opened: {}", 
            file_path.file_name()                     // Same filename extraction as above
                .unwrap_or_default()
                .to_string_lossy()
        );
    }

    // Trigger file dialog to open on next frame
    pub fn open_file_dialog(&mut self) {
        self.file_open_dialog = true;  // UI will check this flag
    }
}  // End of impl block