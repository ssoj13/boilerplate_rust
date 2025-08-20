// Module declarations - include submodules
mod cube;    // cube.rs - 3D cube mesh and rendering
mod shader;  // shader.rs - OpenGL shader utilities

// Import OpenGL context trait and math library
use glow::HasContext;        // Trait that provides OpenGL function methods
use nalgebra_glm as glm;     // Linear algebra library (vectors, matrices) - aliased as 'glm'
use std::sync::Arc;          // Atomic Reference Counter for shared ownership

// Derive Clone trait so we can clone the entire Renderer
#[derive(Clone)]  // Auto-generates clone() method
pub struct Renderer {
    gl: Arc<glow::Context>,  // Shared OpenGL context (Arc allows multiple owners)
    cube: Arc<cube::Cube>,   // Our 3D cube mesh (also shared)
    rotation: f32,           // Current rotation angle in radians
}

// Implementation block for Renderer methods
impl Renderer {
    /// Create a new renderer instance with OpenGL setup
    pub fn new(gl: Arc<glow::Context>) -> Self {
        // Create our cube mesh (wrapped in Arc for sharing)
        let cube = Arc::new(cube::Cube::new(&gl));
        
        // Set up OpenGL state for 3D rendering
        unsafe {  // OpenGL calls are unsafe in Rust
            gl.enable(glow::DEPTH_TEST);    // Enable depth testing (objects behind others are hidden)
            gl.enable(glow::CULL_FACE);     // Enable face culling (don't draw back faces)
            gl.cull_face(glow::BACK);       // Cull back-facing triangles
            gl.front_face(glow::CCW);       // Counter-clockwise triangles are front-facing
        }

        // Return new Renderer instance
        Self {
            gl,               // Store the OpenGL context
            cube,             // Store our cube mesh
            rotation: 0.0,    // Start with no rotation
        }
    }

    /// Update animation state (called each frame if playing)
    pub fn update(&mut self, delta: f32) {
        self.rotation = delta;  // Store new rotation value
    }

    /// Handle window resize - update OpenGL viewport
    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            // Set OpenGL viewport to match new window size
            self.gl.viewport(0, 0, width as i32, height as i32);  // x, y, width, height
        }
    }

    /// Render viewport with egui callback - this is called from the UI paint callback
    pub fn render_viewport(&self, gl: &Arc<glow::Context>, rect: egui::Rect, rotation: f32) {
        use glow::HasContext;  // Import trait in function scope
        
        unsafe {  // All OpenGL calls are unsafe
            // Save current OpenGL viewport so we can restore it later
            let mut current_viewport = [0i32; 4];  // Array to hold [x, y, width, height]
            gl.get_parameter_i32_slice(glow::VIEWPORT, &mut current_viewport);
            
            // Set viewport to match our UI rect (coordinate transformation)
            gl.viewport(
                rect.min.x as i32,    // Left edge of our UI area
                rect.min.y as i32,    // Top edge (note: OpenGL Y is flipped)
                rect.width() as i32,  // Width of our UI area
                rect.height() as i32, // Height of our UI area
            );
            
            // Set up 3D rendering state
            gl.enable(glow::DEPTH_TEST);      // Enable depth testing (closer objects hide farther ones)
            gl.depth_func(glow::LESS);        // Depth test: closer pixels win
            
            // Enable backface culling for performance
            gl.enable(glow::CULL_FACE);       // Don't render triangles facing away
            gl.cull_face(glow::BACK);         // Cull back-facing triangles
            
            // Clear only the depth buffer (color is handled by egui)
            gl.clear(glow::DEPTH_BUFFER_BIT); // Reset depth values for our area

            // Calculate 3D transformation matrices (the math behind 3D graphics!)
            let aspect = rect.width() / rect.height();  // Aspect ratio prevents stretching
            
            // Projection matrix: 3D to 2D (perspective projection)
            let projection = glm::perspective(
                aspect,                   // Aspect ratio (width/height)
                45.0_f32.to_radians(),   // Field of view (45 degrees converted to radians)
                0.1,                     // Near clipping plane
                100.0                    // Far clipping plane
            );
            
            // View matrix: camera position and orientation
            let view = glm::look_at(
                &glm::vec3(2.0, 2.0, 2.0),   // Camera position (eye)
                &glm::vec3(0.0, 0.0, 0.0),   // Look at point (center)
                &glm::vec3(0.0, 1.0, 0.0),   // Up vector (which way is up)
            );
            
            // Model matrix: object transformations (rotation in this case)
            let model = glm::rotate(
                &glm::rotate(
                    &glm::Mat4::identity(),      // Start with identity matrix (no transformation)
                    rotation,                    // Rotate around Y axis
                    &glm::vec3(0.0, 1.0, 0.0),  // Y axis vector
                ),
                rotation * 0.7,                  // Different rotation speed for X axis
                &glm::vec3(1.0, 0.0, 0.0),      // X axis vector
            );

            // Render the cube using the modular Cube struct
            self.cube.render(gl, &projection, &view, &model);  // Pass matrices by reference (&)
            
            // Restore the original viewport (good citizen behavior!)
            gl.viewport(
                current_viewport[0],  // Original x
                current_viewport[1],  // Original y
                current_viewport[2],  // Original width
                current_viewport[3]   // Original height
            );
        }  // End of unsafe block
    }  // End of render_viewport function
}  // End of impl Renderer