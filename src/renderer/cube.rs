// Import OpenGL context trait and linear algebra library
use glow::HasContext;        // Trait providing OpenGL function methods
use nalgebra_glm as glm;     // 3D math library for matrices and vectors

/// 3D Cube mesh with OpenGL resources and shaders
pub struct Cube {
    vao: glow::VertexArray,           // Vertex Array Object (stores vertex attribute setup)
    #[allow(dead_code)]               // Suppress "never read" warning
    vbo: glow::Buffer,                // Vertex Buffer Object (stores vertex data)
    #[allow(dead_code)]               // VBO/EBO are set up once and used by OpenGL directly
    ebo: glow::Buffer,                // Element Buffer Object (stores triangle indices)
    program: glow::Program,           // Compiled shader program
    index_count: i32,                 // Number of indices to draw (36 for a cube)
}

// Implementation of Cube methods
impl Cube {
    /// Create a new cube mesh with compiled shaders and OpenGL buffers
    pub fn new(gl: &glow::Context) -> Self {
        // Vertex shader source code in GLSL (OpenGL Shading Language)
        // Raw string literal r#"..."# allows multiline strings without escaping
        let vertex_shader_source = r#"
            #version 330 core                     // OpenGL 3.3 core profile
            
            // Input vertex attributes (from our VBO)
            layout(location = 0) in vec3 position; // Vertex position (x, y, z)
            layout(location = 1) in vec3 normal;   // Surface normal vector
            layout(location = 2) in vec3 color;    // Vertex color (r, g, b)
            
            // Uniform matrices (same for all vertices in a draw call)
            uniform mat4 u_projection;              // 3D to 2D projection matrix
            uniform mat4 u_view;                    // Camera/view transformation
            uniform mat4 u_model;                   // Object transformation (rotation, etc.)
            
            // Output to fragment shader (interpolated across triangle)
            out vec3 v_normal;       // Normal in world space
            out vec3 v_color;        // Color to be interpolated
            out vec3 v_position;     // Position in world space
            
            void main() {
                // Transform vertex position to world space
                vec4 world_pos = u_model * vec4(position, 1.0);  // 1.0 = homogeneous coordinate
                v_position = world_pos.xyz;  // Pass to fragment shader
                
                // Transform normal to world space (special matrix for normals)
                v_normal = mat3(transpose(inverse(u_model))) * normal;
                v_color = color;  // Pass color through unchanged
                
                // Final vertex position in clip space (required output)
                gl_Position = u_projection * u_view * world_pos;  // MVP transformation
            }
        "#;

        // Fragment shader source code - runs once per pixel
        let fragment_shader_source = r#"
            #version 330 core                // Same OpenGL version as vertex shader
            
            // Input from vertex shader (interpolated values)
            in vec3 v_normal;     // Surface normal (interpolated across triangle)
            in vec3 v_color;      // Vertex color (interpolated across triangle)
            in vec3 v_position;   // World position (interpolated across triangle)
            
            // Output - final pixel color
            out vec4 frag_color;  // RGBA color (red, green, blue, alpha)
            
            void main() {
                // Simple directional lighting calculation
                vec3 light_dir = normalize(vec3(1.0, 1.0, 1.0));  // Light coming from upper-right
                vec3 normal = normalize(v_normal);                // Normalize interpolated normal
                
                // Lighting components
                float ambient = 0.3;                                      // Base lighting (30%)
                float diffuse = max(dot(normal, light_dir), 0.0) * 0.7;  // Directional lighting (70% max)
                float lighting = ambient + diffuse;                      // Combine them
                
                // Final color = vertex color * lighting intensity
                frag_color = vec4(v_color * lighting, 1.0);  // Alpha = 1.0 (fully opaque)
            }
        "#;

        // Compile and link the shaders into a program
        let program = super::shader::create_program(gl, vertex_shader_source, fragment_shader_source)
            .expect("Failed to create shader program");  // Panic if shader compilation fails

        // Cube vertices with positions, normals, and colors
        let vertices: Vec<f32> = vec![
            // Front face (red)
            -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0, 0.0,
             0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0, 0.0,
             0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0, 0.0,
            -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0, 0.0,
            
            // Back face (green)
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0, 0.0,
             0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0, 0.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0, 0.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0, 0.0,
            
            // Top face (blue)
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0, 1.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0, 1.0,
             0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 0.0, 1.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 0.0, 1.0,
            
            // Bottom face (yellow)
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 1.0, 0.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 1.0, 0.0,
             0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0, 0.0,
            
            // Right face (magenta)
             0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0,
             0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0,
             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0, 1.0,
            
            // Left face (cyan)
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 1.0, 1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0, 1.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0, 1.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 1.0, 1.0,
        ];

        let indices: Vec<u32> = vec![
            0,  1,  2,  2,  3,  0,   // Front
            4,  6,  5,  4,  7,  6,   // Back
            8,  9,  10, 10, 11, 8,   // Top
            12, 14, 13, 12, 15, 14,  // Bottom
            16, 17, 18, 18, 19, 16,  // Right
            20, 22, 21, 20, 23, 22,  // Left
        ];

        unsafe {
            let vao = gl.create_vertex_array().expect("Cannot create VAO");
            let vbo = gl.create_buffer().expect("Cannot create VBO");
            let ebo = gl.create_buffer().expect("Cannot create EBO");

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                glow::STATIC_DRAW,
            );

            // Position attribute
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 9 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            // Normal attribute
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 9 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(1);

            // Color attribute
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, 9 * 4, 6 * 4);
            gl.enable_vertex_attrib_array(2);

            gl.bind_vertex_array(None);

            Self {
                vao,
                vbo,
                ebo,
                program,
                index_count: indices.len() as i32,
            }
        }
    }

    /// Render the cube
    pub fn render(&self, gl: &glow::Context, projection: &glm::Mat4, view: &glm::Mat4, model: &glm::Mat4) {
        unsafe {
            gl.use_program(Some(self.program));

            // Set uniforms
            let u_projection = gl.get_uniform_location(self.program, "u_projection");
            gl.uniform_matrix_4_f32_slice(u_projection.as_ref(), false, projection.as_slice());

            let u_view = gl.get_uniform_location(self.program, "u_view");
            gl.uniform_matrix_4_f32_slice(u_view.as_ref(), false, view.as_slice());

            let u_model = gl.get_uniform_location(self.program, "u_model");
            gl.uniform_matrix_4_f32_slice(u_model.as_ref(), false, model.as_slice());

            // Draw
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
            gl.bind_vertex_array(None);
        }
    }
}