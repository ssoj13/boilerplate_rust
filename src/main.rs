// Module declarations - tells Rust to include these files as modules
mod app;        // Application state management (app.rs)
mod config;     // Configuration persistence (config.rs)
mod renderer;   // OpenGL rendering pipeline (renderer/mod.rs + submodules)
mod ui;         // User interface components (ui/mod.rs + submodules)

// External crate imports - like #include in C++ but safer!
use clap::Parser;  // Command-line argument parsing with derive macros
use glow::HasContext;  // Trait that provides OpenGL methods
use glutin::config::ConfigTemplateBuilder;  // Builder pattern for OpenGL config
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};  // OpenGL context setup
use glutin::display::GetGlDisplay;  // Trait for getting GL display
use glutin::prelude::*;  // Import all common glutin traits (the * is a glob import)
use glutin::surface::SwapInterval;  // VSync control
use glutin_winit::{DisplayBuilder, GlWindow};  // Bridge between glutin (OpenGL) and winit (windowing)
use raw_window_handle::HasWindowHandle;  // Low-level window handle access
use std::num::NonZeroU32;  // A u32 that's guaranteed never to be zero (Rust type safety!)
use std::sync::Arc;  // Atomic Reference Counter - shared ownership for multi-threading
use winit::application::ApplicationHandler;  // Trait for handling window events
use winit::dpi::LogicalSize;  // Device-independent size units
use winit::event::{WindowEvent, ElementState};  // Enum of all possible window events
use winit::keyboard::{KeyCode, PhysicalKey};  // Keyboard input handling
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};  // Event loop management
use winit::window::{Window, WindowAttributes};  // Window creation and properties
use winit::dpi::PhysicalPosition;  // Physical position type for window positioning

// Derive macros - Rust's code generation magic!
#[derive(Parser, Debug)]  // Auto-generates argument parsing + debug printing
#[command(author, version, about, long_about = None)]  // CLI metadata
struct Args {
    /// Window width (doc comments become CLI help text!)
    #[arg(short = 'w', long, default_value_t = 1280)]  // -w or --width flag
    width: u32,  // Unsigned 32-bit integer (no negative window sizes!)

    /// Window height
    #[arg(long, default_value_t = 720)]  // Only --height (no short flag to avoid conflict with --help)
    height: u32,
}

// Main application struct - holds all our OpenGL and UI state
struct App {
    // Option<T> means "maybe has a value, maybe None" - Rust's null safety!
    window: Option<Window>,  // The actual window handle
    gl_surface: Option<glutin::surface::Surface<glutin::surface::WindowSurface>>,  // OpenGL drawing surface
    gl_context: Option<glutin::context::PossiblyCurrentContext>,  // OpenGL rendering context
    gl_display: Option<glutin::display::Display>,  // OpenGL display connection
    egui_winit: Option<egui_winit::State>,  // egui's window event handling state
    egui_ctx: egui::Context,  // egui UI context (always present, so no Option needed)
    painter: Option<egui_glow::Painter>,  // egui's OpenGL renderer
    app_state: Option<app::AppState>,  // Our application-specific state
    renderer: Option<renderer::Renderer>,  // Our 3D cube renderer
    gl: Option<Arc<glow::Context>>,  // OpenGL function pointers (Arc = shared ownership)
    config: config::Config,  // Persistent configuration (always present)
}

// Implement the ApplicationHandler trait - this is how we handle window events
impl ApplicationHandler for App {
    // Called when app starts or resumes (on mobile platforms)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let args = Args::parse();  // Parse command line arguments using clap
        
        // Use CLI args if provided, otherwise use config values
        let window_width = if args.width != 1280 { args.width } else { self.config.window.width };
        let window_height = if args.height != 720 { args.height } else { self.config.window.height };
        
        // Builder pattern for window configuration - method chaining!
        let mut window_attributes = WindowAttributes::default()  // Start with defaults
            .with_title("egui OpenGL App")  // Set window title
            .with_inner_size(LogicalSize::new(window_width as f64, window_height as f64));  // Size from config/CLI
        
        // Set window position if saved in config
        if let (Some(x), Some(y)) = (self.config.window.pos_x, self.config.window.pos_y) {
            window_attributes = window_attributes.with_position(PhysicalPosition::new(x, y));
        }
        
        // Set maximized state if saved in config
        if self.config.window.maximized {
            window_attributes = window_attributes.with_maximized(true);
        }

        // OpenGL configuration template - more builder pattern magic
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)  // 8-bit alpha channel for transparency
            .with_transparency(false);  // We don't need window transparency

        // Builder for the OpenGL display - combines window + OpenGL setup
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        // Build the display and window - returns a tuple (Rust loves tuples!)
        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                // Closure (anonymous function) to pick the best OpenGL config
                configs
                    .reduce(|accum, config| {  // Iterator reduce - like fold but simpler
                        // Pick config with more MSAA samples (anti-aliasing)
                        if config.num_samples() > accum.num_samples() {
                            config  // Return the better config
                        } else {
                            accum   // Keep the accumulator
                        }
                    })
                    .unwrap()  // Panic if no configs found (unwrap = "give me the value or crash")
            })
            .unwrap();  // Another unwrap - crash if display building fails

        let window = window.unwrap();  // Extract window from Option (it should always be Some here)
        
        // Get the low-level window handle for OpenGL context creation
        let raw_window_handle = window.window_handle()  // Result<WindowHandle, _>
            .ok()  // Convert Result to Option (ignore errors)
            .map(|wh| wh.as_raw());  // Option::map applies function if Some, returns None if None
        
        let gl_display = gl_config.display();  // Get the OpenGL display from config
        
        // Build OpenGL context attributes - requesting OpenGL 4.5
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 5))))  // Request OpenGL 4.5 specifically
            .build(raw_window_handle);  // Associate with our window

        // Create OpenGL context - this is where the magic happens!
        let not_current_context = unsafe {  // unsafe because we're dealing with C APIs
            gl_display
                .create_context(&gl_config, &context_attributes)
                .expect("failed to create context")  // expect = unwrap with custom error message
        };

        // Create surface attributes for drawing
        let attrs = window
            .build_surface_attributes(Default::default())  // Default::default() = default values
            .expect("Failed to build surface attributes");
            
        // Create the actual drawing surface - where pixels get rendered
        let gl_surface = unsafe {  // More unsafe C API calls
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .expect("Failed to create surface")
        };

        // Make the context current - activate it for rendering
        let gl_context = not_current_context  // Context starts as "not current"
            .make_current(&gl_surface)  // Bind it to our surface = "current context"
            .expect("Failed to make context current");

        // Create the OpenGL function loader - this is where we get all OpenGL functions!
        let gl = Arc::new(unsafe {  // Arc = Atomic Reference Counted (shared ownership)
            glow::Context::from_loader_function_cstr(|s| {  // Closure that loads OpenGL functions
                gl_display.get_proc_address(s) as *const _  // Get function pointer, cast to generic pointer
            })
        });

        // Disable VSync on Wayland to avoid blocking issues that can cause segfaults
        // See: https://github.com/rust-windowing/winit/issues/2891
        if let Err(e) = gl_surface.set_swap_interval(&gl_context, SwapInterval::DontWait) {
            eprintln!("Failed to disable vsync: {:?}", e);  // eprintln! = print to stderr
            // Note: if let Err(e) = ... is pattern matching on Result<T, E>
        }

        // Create egui context - the heart of our UI system
        let egui_ctx = egui::Context::default();
        
        // Create egui-winit bridge - connects egui to window events
        let egui_winit = egui_winit::State::new(
            egui_ctx.clone(),  // Clone the context (Arc internally, so cheap to clone)
            egui::ViewportId::ROOT,  // Main viewport ID
            event_loop,  // Reference to event loop
            Some(window.scale_factor() as f32),  // DPI scaling factor
            None,  // No custom theme
            None,  // No custom font definitions
        );
        
        // Create egui's OpenGL painter - renders UI to OpenGL
        let painter = egui_glow::Painter::new(
            gl.clone(),  // Clone our OpenGL context (Arc makes this cheap)
            "",  // Empty shader prefix
            None,  // No custom shader header
            false,  // Don't check for GL errors every call (performance)
        ).expect("Failed to create egui painter");

        // Initialize our application state and 3D renderer
        let app_state = app::AppState::new();  // Create new app state with defaults
        let renderer = renderer::Renderer::new(gl.clone());  // Create cube renderer

        // Store everything in our App struct - moving ownership from local variables
        self.window = Some(window);          // Some() wraps the value in Option
        self.gl_surface = Some(gl_surface);
        self.gl_context = Some(gl_context);
        self.gl_display = Some(gl_display);
        self.egui_winit = Some(egui_winit);
        self.painter = Some(painter);
        self.app_state = Some(app_state);
        self.renderer = Some(renderer);
        self.gl = Some(gl);
    }  // End of resumed() function

    // Handle window events - called for every window event!
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId, event: WindowEvent) {
        // Early returns with let-else pattern (Rust 1.65+) - super clean!
        let Some(window) = &self.window else { return };           // Borrow window or return early
        let Some(egui_winit) = &mut self.egui_winit else { return };  // Mutable borrow for event handling
        let Some(painter) = &mut self.painter else { return };
        let Some(gl_surface) = &self.gl_surface else { return };
        let Some(gl_context) = &self.gl_context else { return };
        let Some(app_state) = &mut self.app_state else { return };    // Mutable - we'll change app state
        let Some(renderer) = &mut self.renderer else { return };      // Mutable - renderer might update
        let Some(gl) = &self.gl else { return };

        // Let egui handle the event first - returns whether it wants the event
        let _ = egui_winit.on_window_event(window, &event);  // _ = ignore return value

        // Pattern matching on the event type - like switch/case but more powerful!
        match event {
            WindowEvent::CloseRequested => {  // User clicked X button
                event_loop.exit();  // Quit the application
            }
            WindowEvent::KeyboardInput { event: key_event, .. } => {  // Keyboard key pressed/released
                // Only handle key press events (not releases)
                if key_event.state == ElementState::Pressed {
                    match key_event.physical_key {  // Match on physical key codes
                        PhysicalKey::Code(KeyCode::Space) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                            // Space or Up Arrow: Toggle play/pause
                            app_state.toggle_playing();
                            window.request_redraw();  // Update UI immediately
                        }
                        PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyR) => {
                            // Left Arrow or R: Reset animation
                            app_state.reset();
                            window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::ArrowRight) => {
                            // Right Arrow: Step one frame
                            app_state.step();
                            window.request_redraw();
                        }
                        PhysicalKey::Code(KeyCode::Escape) => {
                            // Escape: Quit application
                            event_loop.exit();
                        }
                        _ => {}  // Ignore all other keys
                    }
                }
            }
            WindowEvent::Resized(size) => {  // Window size changed
                // Resize OpenGL surface to match new window size
                gl_surface.resize(gl_context, 
                    NonZeroU32::new(size.width).unwrap(),   // NonZeroU32 can't be 0
                    NonZeroU32::new(size.height).unwrap()
                );
                renderer.resize(size.width, size.height);  // Tell our renderer about new size
                
                // Save new window size to config
                self.config.update_window_size(size.width, size.height);
                self.config.save();  // Persist immediately
            }
            WindowEvent::Moved(position) => {  // Window position changed
                // Save new window position to config  
                self.config.update_window_pos(position.x, position.y);
                self.config.save();  // Persist immediately
            }
            WindowEvent::RedrawRequested => {  // Time to draw a frame!
                let size = window.inner_size();  // Get current window size
                
                // Get input state from winit and give it to egui
                let raw_input = egui_winit.take_egui_input(window);
                
                // Run egui for one frame - the closure builds the UI
                let full_output = self.egui_ctx.run(raw_input, |ctx| {
                    // This closure is where we build our entire UI!
                    ui::show_ui(ctx, app_state, renderer, size.width, size.height);
                });  // Returns what egui wants to draw
                
                // Handle platform-specific output (cursor changes, etc.)
                egui_winit.handle_platform_output(window, full_output.platform_output);
                
                // Convert egui shapes into renderable triangles (tessellation)
                let primitives = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
                
                // Clear the screen with dark gray background
                unsafe {  // OpenGL calls are unsafe in Rust
                    gl.clear_color(0.1, 0.1, 0.1, 1.0);  // R, G, B, A values
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);  // Clear both color and depth
                }
                
                // Render egui (including 3D viewport via callbacks)
                painter.paint_primitives(
                    [size.width as u32, size.height as u32],  // Screen size as array
                    full_output.pixels_per_point,  // DPI scaling factor
                    &primitives,  // The tessellated UI geometry
                );
                
                // Handle texture updates - egui manages textures for images/fonts
                for (id, image_delta) in &full_output.textures_delta.set {
                    painter.set_texture(*id, &image_delta);  // * dereferences the id
                }
                
                // Free textures that are no longer needed
                for id in &full_output.textures_delta.free {
                    painter.free_texture(*id);
                }

                // Swap front and back buffers - make our drawing visible!
                gl_surface.swap_buffers(gl_context).expect("Failed to swap buffers");
                
                // Request another frame immediately (continuous rendering)
                window.request_redraw();
            }
            _ => {}  // Ignore all other window events with wildcard pattern
        }
    }  // End of window_event function

    // Called when event loop is about to wait for more events
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Keep requesting redraws for smooth animation
        if let Some(window) = &self.window {  // if let = safe optional access
            window.request_redraw();  // Queue another RedrawRequested event
        }
    }
}  // End of ApplicationHandler impl

// The main function - entry point of our program!
fn main() {
    // Load configuration first - this handles file I/O and default creation
    let config = config::Config::load();
    
    // Create the event loop - heart of any GUI application
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);  // Don't block, poll for events continuously
    
    // Create our application with all fields as None initially
    let mut app = App {
        window: None,           // Will be set in resumed()
        gl_surface: None,       // Window might not be ready yet
        gl_context: None,       // OpenGL setup happens later
        gl_display: None,
        egui_winit: None,
        egui_ctx: egui::Context::default(),  // egui context can be created immediately
        painter: None,
        app_state: None,
        renderer: None,
        gl: None,
        config,                 // Store loaded configuration
    };
    
    // Run the event loop - this takes ownership of app and never returns!
    eprintln!("Starting event loop...");
    let result = event_loop.run_app(&mut app);
    eprintln!("Event loop ended with result: {:?}", result);
    eprintln!("Main function ending...");
}  // End of main - program ends here (if it ever reaches here)