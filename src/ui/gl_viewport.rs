// Import our app state, renderer, and Arc for shared ownership
use crate::app::AppState;
use crate::renderer::Renderer;
use std::sync::Arc;  // Atomic Reference Counter for thread-safe shared ownership

/// Display the OpenGL viewport with proper callback rendering
/// This is where our 3D cube gets rendered within the egui UI!
pub fn show_viewport(ui: &mut egui::Ui, renderer: &Renderer, app_state: &AppState, rect: egui::Rect) {
    // Allocate space in the UI for our 3D viewport
    let response = ui.allocate_rect(rect, egui::Sense::hover());  // Sense::hover = track mouse hover
    
    // Calculate rotation based on frame count (makes cube spin)
    let rotation = app_state.frame_count as f32 * 0.01;  // Convert to f32 and scale down
    
    // Clone renderer for use in the callback closure
    let renderer_clone = renderer.clone();  // Clone is cheap because Renderer uses Arc internally
    
    // Create egui paint callback - this is where OpenGL rendering happens!
    let callback = egui::PaintCallback {
        rect: response.rect,  // Where to render in screen coordinates
        callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
            // This closure runs during egui's paint phase
            // move = take ownership of renderer_clone and rotation
            
            // Get the OpenGL context from egui's painter
            let gl = painter.gl();  // This is our glow::Context
            
            // Render our 3D cube using the modular renderer
            renderer_clone.render_viewport(gl, response.rect, rotation);
        })),
    };
    
    // Add our callback to egui's paint list
    ui.painter().add(callback);  // egui will call our callback during rendering
}  // End of show_viewport function

