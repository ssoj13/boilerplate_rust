// Import OpenGL context trait
use glow::HasContext;

/// Compile and link shader program from vertex and fragment shader source code
/// Returns Result<Program, String> - either success with program or error message
pub fn create_program(
    gl: &glow::Context,    // OpenGL context reference
    vertex_source: &str,   // Vertex shader GLSL source code (&str = string slice)
    fragment_source: &str, // Fragment shader GLSL source code
) -> Result<glow::Program, String> {  // Result = either Ok(Program) or Err(String)
    unsafe {  // OpenGL calls are unsafe
        // Create a new shader program object
        let program = gl.create_program().expect("Cannot create program");

        // Compile both shaders (? operator propagates errors up)
        let vertex_shader = compile_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
        let fragment_shader = compile_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;

        // Attach shaders to the program
        gl.attach_shader(program, vertex_shader);    // Add vertex shader
        gl.attach_shader(program, fragment_shader);  // Add fragment shader
        gl.link_program(program);                    // Link them together into executable

        // Check if linking succeeded
        if !gl.get_program_link_status(program) {  // Returns false if linking failed
            let log = gl.get_program_info_log(program);  // Get error details
            gl.delete_program(program);                  // Clean up failed program
            return Err(format!("Program linking failed: {}", log));  // Return error
        }

        // Clean up individual shaders (program keeps the compiled code)
        gl.delete_shader(vertex_shader);    // Delete vertex shader object
        gl.delete_shader(fragment_shader);  // Delete fragment shader object

        Ok(program)  // Return successfully linked program
    }  // End of unsafe block
}  // End of create_program function

/// Compile individual shader from GLSL source code
/// Private function (no pub) - only used internally by create_program
fn compile_shader(
    gl: &glow::Context,  // OpenGL context
    shader_type: u32,    // Either VERTEX_SHADER or FRAGMENT_SHADER
    source: &str,        // GLSL source code as string
) -> Result<glow::Shader, String> {  // Returns compiled shader or error message
    unsafe {
        // Create a new shader object of the specified type
        let shader = gl.create_shader(shader_type).expect("Cannot create shader");
        
        // Upload the source code to the shader object
        gl.shader_source(shader, source);
        
        // Compile the shader from source code to GPU instructions
        gl.compile_shader(shader);

        // Check if compilation succeeded
        if !gl.get_shader_compile_status(shader) {  // Returns false if compilation failed
            let log = gl.get_shader_info_log(shader);  // Get compiler error details
            gl.delete_shader(shader);                  // Clean up failed shader
            return Err(format!("Shader compilation failed: {}", log));  // Return error
        }

        Ok(shader)  // Return successfully compiled shader
    }  // End of unsafe block
}  // End of compile_shader function