# CLAUDE.md - Project State Documentation

## Current Implementation Status

This document captures the current state of the egui OpenGL application for future development continuation.

## Architecture Overview

### Technology Stack
- **Rust** with Cargo build system
- **egui 0.29** - Immediate mode GUI framework
- **egui_glow 0.29** - OpenGL backend for egui
- **winit 0.30** - Cross-platform window creation
- **glutin 0.32** - OpenGL context management
- **glow 0.14** - Safe OpenGL bindings
- **rfd 0.15** - Native file dialogs
- **nalgebra-glm** - Linear algebra for 3D transformations
- **OpenGL 4.5** - Modern graphics API

### Project Structure
```
testbed2_rust/
├── src/
│   ├── main.rs          # Window creation, event loop, OpenGL context
│   ├── app.rs           # Application state management
│   ├── ui/
│   │   ├── mod.rs       # Main UI orchestration
│   │   ├── menu.rs      # File/Help menu with file dialog
│   │   ├── toolbar.rs   # Open/Play/Step/Reset controls
│   │   ├── statusbar.rs # Frame counter and mouse position
│   │   └── gl_viewport.rs # 3D viewport with cube rendering
│   └── renderer/
│       ├── mod.rs       # Renderer state and matrix setup
│       ├── cube.rs      # Cube mesh and rendering
│       └── shader.rs    # Shader compilation utilities
├── build.cmd            # Windows build script
├── build.sh            # Linux build script
└── Cargo.toml          # Dependencies configuration
```

## Key Implementation Details

### 1. OpenGL Integration with egui
The 3D viewport uses egui's `PaintCallback` system to render within the GUI paint pipeline:

```rust
// src/ui/gl_viewport.rs
let callback = egui::PaintCallback {
    rect: response.rect,
    callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
        let gl = painter.gl();
        render_cube_directly(gl, response.rect, rotation);
    })),
};
```

This approach ensures proper OpenGL state management and prevents rendering conflicts.

### 2. Window and Context Creation
Using winit + glutin for cross-platform OpenGL context:
- OpenGL 4.5 context requested
- VSync enabled via SwapInterval
- Proper surface configuration for Windows/Linux

### 3. Animation System
- Frame counter in `AppState` drives rotation
- Play/Pause/Step/Reset controls modify state
- Continuous repaint requested when playing
- Rotation calculated as: `frame_count * 0.01`

### 4. File Dialog Integration
- Uses `rfd` crate for native file dialogs
- Triggered from both menu and toolbar
- Stores selected file path in `AppState::current_file`

## Current Features

### Implemented ✅
- Cross-platform window creation (Windows/Linux)
- OpenGL 4.5 context initialization
- egui GUI with menu bar, toolbar, status bar
- 3D viewport with rotating colored cube
- File open dialog (native)
- Animation controls (Play/Pause/Step/Reset)
- Frame counter and mouse position tracking
- Command-line arguments for window size
- Build scripts for both platforms

### Cube Rendering Details
- 8 vertices with RGB colors
- 36 indices for 12 triangles (6 faces)
- GLSL 330 core shaders
- MVP matrix transformations
- Depth testing and backface culling enabled
- Camera positioned at (2, 2, 2) looking at origin

## Known Working State

### Build Commands
```bash
# Windows
cargo build --release
./build.cmd

# Linux
cargo build --release
./build.sh
```

### Run Commands
```bash
# Default 1280x720 window
./target/release/egui_opengl_app

# Custom size
./target/release/egui_opengl_app -w 1920 -h 1080
```

## Future Integration Points

### CUDA Texture Rendering
The current cube rendering in `gl_viewport.rs` is designed to be replaced:

1. **Current Implementation**: Direct cube rendering in callback
2. **Future CUDA Integration**:
   - Create OpenGL texture
   - Register with CUDA for interop
   - Render CUDA output to texture
   - Display texture in viewport

The `render_cube_directly()` function can be replaced with:
```rust
render_cuda_texture(gl, cuda_texture_id, rect);
```

### Extensibility
- Renderer module is isolated and replaceable
- UI components are modular
- State management supports additional fields
- Callback system allows custom rendering

## Dependencies Lock

Critical version requirements:
- egui/egui_glow/egui-winit must match (currently 0.29)
- winit 0.30 required for current event loop API
- glutin 0.32 + glutin-winit 0.5 for context creation
- glow 0.14 for OpenGL bindings

## Performance Notes

- VSync enabled to prevent tearing
- Continuous redraw only when animation playing
- Efficient callback-based rendering
- OpenGL resources cleaned up after each frame
- Scissor test used to clip viewport rendering

## Error Handling

Current error handling includes:
- Graceful VSync fallback if not supported
- Context creation validation
- Shader compilation checks
- Surface resize handling

## Testing Notes

The application has been verified to:
- Build on Windows 11
- Display rotating colored cube
- Open native file dialogs
- Respond to all UI controls
- Handle window resizing correctly
- Process command-line arguments

## Development Tips

1. **Adding New UI Elements**: Modify `ui/mod.rs` and create new module
2. **Changing 3D Content**: Replace `render_cube_directly()` in `gl_viewport.rs`
3. **State Management**: Extend `AppState` in `app.rs`
4. **New Menu Items**: Update `menu.rs` with actions
5. **Toolbar Buttons**: Add to `toolbar.rs` with handlers

## Debug Information

To enable debug output:
- Viewport coordinates printed when implemented
- OpenGL errors can be checked with `gl.get_error()`
- Shader compilation logs available via `gl.get_shader_info_log()`

## Last Working Commit State

- File dialog integration complete
- 3D cube rendering via PaintCallback
- All UI controls functional
- Cross-platform build scripts ready
- Documentation created (README.md, CLAUDE.md)