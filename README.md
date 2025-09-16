# egui OpenGL App

A cross-platform Rust application featuring egui GUI with integrated OpenGL 3D viewport. Built for both Windows and Linux with modern OpenGL 4.5 support.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![OpenGL](https://img.shields.io/badge/OpenGL-%23FFFFFF.svg?style=for-the-badge&logo=opengl)

## Features

- 🎨 **Modern GUI** - Clean interface with menu bar, toolbar, and status bar using egui
- 🎲 **3D Viewport** - Integrated OpenGL rendering with rotating cube demo
- 📁 **File Dialog** - Native file picker integration
- 🎮 **Animation Controls** - Play/Pause, Step, and Reset controls
- 🖥️ **Cross-Platform** - Builds on Windows and Linux
- ⚡ **High Performance** - OpenGL 4.5 with efficient rendering pipeline

## Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- OpenGL 4.5 capable GPU
- Platform-specific dependencies:
  
  **Windows:** Visual Studio Build Tools or MSVC
  
  **Linux:** 
  ```bash
  sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
  ```

### Build & Run

```bash
# Clone the repository
git clone <your-repo>
cd testbed2_rust

# Build the project
cargo build --release

# Or use the build scripts
./build.cmd  # Windows
./build.sh   # Linux

# Run the application
./target/release/egui_opengl_app
```

### Command Line Options

```bash
egui_opengl_app [OPTIONS]

Options:
  -w, --width <WIDTH>     Window width [default: 1280]
      --height <HEIGHT>   Window height [default: 720]
  -h, --help             Print help
  -V, --version          Print version
```

## Architecture

The application uses a modular architecture:

- **`main.rs`** - Window management and event loop using winit
- **`app.rs`** - Application state management
- **`ui/`** - egui interface components (menu, toolbar, status bar)
- **`renderer/`** - OpenGL rendering pipeline with cube demo

### Key Technologies

- **egui** - Immediate mode GUI framework
- **glow** - OpenGL bindings
- **winit** - Cross-platform windowing
- **glutin** - OpenGL context creation
- **rfd** - Native file dialogs

## 3D Viewport

The 3D viewport demonstrates OpenGL integration with egui:
- Renders during egui's paint phase using callbacks
- Isolated rendering function ready for custom implementations
- Perfect for CUDA texture integration or custom 3D content

## Application Flow & Cyclogram

### Program Lifecycle

```
main() 
├── EventLoop::new()                    // Create window event loop
├── App::new()                          // Initialize app with None fields
└── event_loop.run_app(&mut app)        // Start event loop (never returns)
    │
    ├── ApplicationHandler::resumed()    // Called on app start
    │   ├── Args::parse()               // Parse CLI arguments
    │   ├── WindowAttributes::default() // Configure window
    │   ├── DisplayBuilder::new()       // Setup OpenGL display
    │   ├── gl_display.create_context() // Create OpenGL context
    │   ├── egui::Context::default()    // Initialize egui
    │   ├── egui_winit::State::new()    // Connect egui to winit
    │   ├── egui_glow::Painter::new()   // Create egui renderer
    │   ├── AppState::new()             // Initialize app state
    │   └── Renderer::new()             // Create 3D renderer
    │       └── Cube::new()             // Create cube mesh + shaders
    │
    └── [Event Loop - runs continuously]
        ├── ApplicationHandler::window_event()
        │   ├── egui_winit.on_window_event()     // Let egui handle events
        │   └── match event:
        │       ├── CloseRequested → exit()
        │       ├── Resized → gl_surface.resize() + renderer.resize()
        │       └── RedrawRequested → [RENDER FRAME]
        │
        └── ApplicationHandler::about_to_wait()
            └── window.request_redraw()          // Queue next frame
```

### Frame Rendering Cycle (RedrawRequested)

```
RedrawRequested Event
├── egui_winit.take_egui_input()        // Get input from winit
├── egui_ctx.run(|ctx| { ... })        // Build UI for this frame
│   └── ui::show_ui()                   // Main UI function
│       ├── ctx.pointer_hover_pos()     // Update mouse position
│       ├── show_menu()                 // File/Help menu
│       │   ├── rfd::FileDialog::new()  // File picker (if requested)
│       │   └── egui::TopBottomPanel::top("menu_bar")
│       ├── show_toolbar()              // Play/Step/Reset controls
│       │   └── egui::TopBottomPanel::top("toolbar")
│       │       ├── ui.button("📁 Open")
│       │       ├── ui.button("▶ Play") // or "⏸ Pause"
│       │       ├── ui.button("⏭ Step")
│       │       └── ui.button("⏮ Reset")
│       ├── egui::CentralPanel::default() // 3D viewport area
│       │   └── gl_viewport::show_viewport()
│       │       ├── ui.allocate_rect()   // Reserve space for 3D
│       │       └── egui::PaintCallback  // OpenGL rendering callback
│       │           └── renderer.render_viewport()
│       │               ├── gl.viewport()        // Set OpenGL viewport
│       │               ├── gl.enable(DEPTH_TEST) // Setup 3D state
│       │               ├── glm::perspective()   // Calculate projection matrix
│       │               ├── glm::look_at()       // Calculate view matrix  
│       │               ├── glm::rotate()        // Calculate model matrix
│       │               └── cube.render()        // Render the cube
│       │                   ├── gl.use_program() // Activate shaders
│       │                   ├── gl.uniform_matrix_4_f32_slice() // Upload matrices
│       │                   └── gl.draw_elements() // Draw triangles
│       ├── show_statusbar()            // Bottom status bar
│       │   └── egui::TopBottomPanel::bottom("status_bar")
│       └── [Animation Update]
│           ├── if app_state.playing:
│           │   ├── app_state.step()    // Increment frame counter
│           │   ├── renderer.update()   // Update rotation
│           │   └── ctx.request_repaint() // Queue next frame
├── egui_winit.handle_platform_output() // Handle cursor changes, etc.
├── egui_ctx.tessellate()               // Convert UI to triangles
├── gl.clear()                          // Clear screen
├── painter.paint_primitives()          // Render egui UI (+ 3D callback)
├── [Handle texture updates]            // egui font/image textures
└── gl_surface.swap_buffers()           // Present frame to screen
```

### Key Data Flow

```
User Input → winit → egui_winit → egui → UI Components → AppState
                                    ↓
AppState → Renderer → Cube → OpenGL → GPU → Screen
```

### Module Dependencies

```
main.rs
├── app.rs                    // AppState management
├── ui/mod.rs                 // UI orchestration
│   ├── ui/gl_viewport.rs     // 3D viewport
│   └── [menu, toolbar, statusbar functions]
├── renderer/mod.rs           // 3D rendering
│   ├── renderer/cube.rs      // Cube mesh + rendering
│   └── renderer/shader.rs    // GLSL shader compilation
└── tests/                    // Testing infrastructure
    └── integration_tests.rs  // Integration tests
```

## Testing

This project includes comprehensive testing infrastructure for a GUI/OpenGL application.

### Running Tests Locally

```bash
# Unit and integration tests
cargo test

# Specific test categories
cargo test --lib                    # Unit tests only
cargo test --test integration_tests # Integration tests only

# Code quality checks
cargo fmt --check                   # Format verification
cargo clippy -- -D warnings        # Linting with strict warnings

# Security audit
cargo install cargo-audit
cargo audit
```

### Test Categories

#### **Unit Tests**
- Application state management
- String formatting and path operations
- Mathematical calculations (rotation, aspect ratio)
- Error handling patterns

#### **Integration Tests** (`tests/integration_tests.rs`)
- CLI argument parsing (`--help`, `--version`, custom dimensions)
- Binary existence and execution
- Graceful shutdown handling
- Cross-platform compatibility
- Performance characteristics (startup time, binary size)


### GitHub Actions CI/CD

The project uses a comprehensive CI/CD pipeline with multiple workflows:

#### **Continuous Integration** (`.github/workflows/ci.yml`)
- **Multi-platform testing:** Ubuntu, Windows, macOS
- **Rust version compatibility:** stable, beta
- **Automated quality checks:** clippy, rustfmt
- **Build verification** with artifact uploads
- **CLI testing** with proper GUI app handling

#### **Security Monitoring** (`.github/workflows/security.yml`)
- **Weekly vulnerability scans** with cargo-audit
- **License compliance checking**
- **Dependency freshness monitoring**
- **CodeQL static analysis**

#### **Advanced Testing** (`.github/workflows/nightly.yml`)
- **Rust nightly compatibility**
- **Memory leak detection** with Valgrind
- **Minimal dependency versions testing**
- **Fuzz testing** capabilities
- **Performance regression tracking**

#### **Release Automation** (`.github/workflows/release.yml`)
- **Multi-platform binary builds**
- **Automated GitHub releases**
- **Asset packaging** with documentation
- **Post-release verification**

**Note:** For the release workflow to work properly when pushing tags, you need to set up a Personal Access Token (PAT) as a repository secret. See `.github/README.md` for detailed instructions.

### GUI Application Testing Challenges

Special considerations for testing a GUI/OpenGL application:

#### **Headless Testing**
- Uses `xvfb` on Linux for GUI apps without display
- Timeout-based testing to avoid hanging processes
- CLI-only tests where possible (`--help`, `--version`)

#### **OpenGL Dependencies**
- Proper system library installation in CI
- Cross-platform graphics driver compatibility
- Fallback testing strategies for headless environments

#### **Performance Testing**
- Binary size monitoring (debug: <100MB threshold)
- Startup time benchmarks (target: <5 seconds)
- Memory usage validation with Valgrind

### Test Data and Fixtures

The application includes test scenarios for:
- **Valid CLI arguments:** `-w 800 --height 600`
- **Invalid arguments:** Error handling verification
- **File operations:** Path creation and filename extraction
- **Animation cycles:** Frame counting and rotation math
- **State management:** Play/pause/step/reset functionality

### Local Development Testing

Before submitting PRs, run the full test suite:

```bash
# Quick verification (under 30 seconds)
cargo check && cargo clippy && cargo fmt --check

# Full test suite
cargo test

# Platform-specific testing
cargo test --target x86_64-pc-windows-msvc  # Windows
cargo test --target x86_64-unknown-linux-gnu # Linux
cargo test --target x86_64-apple-darwin     # macOS
```

### CI/CD Status Badges

Monitor build status and quality metrics:

[![CI](https://github.com/username/repo/workflows/CI/badge.svg)](https://github.com/username/repo/actions/workflows/ci.yml)
[![Security](https://github.com/username/repo/workflows/Security/badge.svg)](https://github.com/username/repo/actions/workflows/security.yml)

*Replace `username/repo` with actual GitHub repository path*

### Debugging Test Failures

Common issues and solutions:

1. **OpenGL context errors:** Ensure proper system dependencies
2. **GUI tests hanging:** Check timeout configurations
3. **Cross-platform path issues:** Use `std::path::PathBuf` consistently
4. **Memory test failures:** Review Valgrind suppressions for false positives

The testing infrastructure is designed to catch issues early while maintaining fast feedback loops for developers.

## Development

The renderer is designed to be easily replaceable. The cube rendering in `src/ui/gl_viewport.rs` can be swapped with:
- CUDA-generated textures
- Custom 3D scenes
- Video processing pipelines
- Scientific visualizations

## License

MIT

## Contributing

Pull requests welcome! Please ensure cross-platform compatibility and run the full test suite locally before submitting.

