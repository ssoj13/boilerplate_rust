// Module declaration - include gl_viewport.rs as a submodule
mod gl_viewport;

// Import types from our crate (crate = current package)
use crate::app::AppState;
use crate::renderer::Renderer;

/// Main UI rendering function - called once per frame to build the entire UI
pub fn show_ui(ctx: &egui::Context, app_state: &mut AppState, renderer: &mut Renderer, _window_width: u32, _window_height: u32) {
    // _ prefix means "unused parameter" - we don't use window dimensions here
    
    // Update mouse position from egui context
    if let Some(pos) = ctx.pointer_hover_pos() {  // pointer_hover_pos() returns Option<Pos2>
        app_state.update_mouse(pos.x, pos.y);     // Extract x, y from Pos2
    }

    // Build UI components in order (top to bottom)
    show_menu(ctx, app_state);       // File/Help menu at top
    show_toolbar(ctx, app_state);    // Play/Step/Reset buttons below menu
    
    // Central panel with OpenGL viewport - takes remaining space
    egui::CentralPanel::default().show(ctx, |ui| {  // .show() takes a closure for UI building
        let available_rect = ui.available_rect_before_wrap();  // Get remaining space
        gl_viewport::show_viewport(ui, renderer, app_state, available_rect);  // Render 3D cube here
    });

    show_statusbar(ctx, app_state);  // Status info at bottom

    // Handle animation updates
    if app_state.playing {  // Only update if animation is playing
        app_state.step();                                     // Advance frame counter
        renderer.update(app_state.frame_count as f32 * 0.01); // Update renderer rotation (as = type cast)
        ctx.request_repaint();                                // Tell egui to redraw next frame
    }
}  // End of show_ui function

/// Display the toolbar with play controls
fn show_toolbar(ctx: &egui::Context, app_state: &mut AppState) {
    // TopBottomPanel creates a panel that docks to top or bottom
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {  // "toolbar" = unique ID
        ui.horizontal(|ui| {  // Layout children horizontally (left to right)
            // Customize button appearance
            ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);  // vec2 = 2D vector (x, y)
            
            // File open button with emoji icon
            if ui.button("📁 Open").clicked() {  // .clicked() returns bool
                app_state.open_file_dialog();    // Set flag to open dialog
            }

            ui.separator();  // Visual separator line

            // Dynamic play/pause button text based on state
            let play_text = if app_state.playing { "⏸ Pause" } else { "▶ Play" };
            if ui.button(play_text).clicked() {
                app_state.toggle_play();  // Switch between play/pause
                // Update status text to reflect new state
                app_state.status_text = if app_state.playing { 
                    "Playing".to_string()   // to_string() converts &str to String
                } else { 
                    "Paused".to_string() 
                };
            }

            // Step forward one frame button
            if ui.button("⏭ Step").clicked() {
                app_state.step();  // Advance by exactly one frame
                app_state.status_text = format!("Stepped to frame {}", app_state.frame_count);
            }

            // Reset animation to beginning button
            if ui.button("⏮ Reset").clicked() {
                app_state.reset();  // Back to frame 0 and stop playing
                app_state.status_text = "Reset".to_string();
            }
        });  // End of horizontal layout
    });  // End of top panel
}  // End of show_toolbar function

/// Display the status bar at bottom of window
fn show_statusbar(ctx: &egui::Context, app_state: &AppState) {
    // Bottom panel - docks to bottom of window
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {  // Horizontal layout for status items
            ui.label(app_state.get_status());  // Show the formatted status string
        });
    });
}  // End of show_statusbar function

/// Display the menu bar with File and Help menus
fn show_menu(ctx: &egui::Context, app_state: &mut AppState) {
    // Handle file dialog if the flag is set (checked once per frame)
    if app_state.file_open_dialog {
        app_state.file_open_dialog = false;  // Reset flag immediately
        
        // rfd = Rust File Dialog - native OS file picker
        if let Some(path) = rfd::FileDialog::new()  // Builder pattern for dialog config
            .add_filter("All Files", &["*"])                                    // File type filters
            .add_filter("Text Files", &["txt"])                                 // &["..."] = slice of string literals
            .add_filter("Data Files", &["json", "csv", "xml"])
            .add_filter("Image Files", &["png", "jpg", "jpeg", "bmp", "gif"])
            .set_title("Open File")                                             // Dialog title
            .pick_file()                                                        // Show dialog and return Option<PathBuf>
        {
            // User selected a file - the Some(path) case
            app_state.set_current_file(path);  // Store the selected file
        } else {
            // User cancelled dialog - the None case
            app_state.status_text = "File open cancelled".to_string();
        }
    }

    // Create menu bar at top of window
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {  // egui's menu bar container
            // File menu dropdown
            ui.menu_button("File", |ui| {  // ui parameter is for inside the dropdown
                if ui.button("Open").clicked() {
                    app_state.open_file_dialog();  // Set flag to show dialog next frame
                    ui.close_menu();               // Close dropdown after click
                }
                
                ui.separator();  // Visual separator in menu
                
                if ui.button("Exit").clicked() {
                    std::process::exit(0);  // Immediately terminate program with exit code 0
                }
            });

            // Help menu dropdown
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    app_state.status_text = "egui OpenGL App v0.1.0".to_string();
                    ui.close_menu();  // Close dropdown
                }
            });
        });  // End of menu bar
    });  // End of top panel
}  // End of show_menu function