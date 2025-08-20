#!/bin/bash
echo "Building egui OpenGL application..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi
echo "Build successful! Binary is at target/release/egui_opengl_app"