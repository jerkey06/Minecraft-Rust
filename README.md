# Minecraft Clone in Rust

This project is a Minecraft clone written in Rust, using `wgpu` for rendering. It serves as a learning playground for computer graphics, game development, and the Rust ecosystem.

## Features

*   **3D Rendering:** Uses `wgpu` to render a simple 3D scene with a rotating cube.
*   **Debug Overlay:** Displays an F3-style debug overlay that can be toggled with the F3 key. The overlay shows:
    *   FPS (Frames Per Second)
    *   CPU usage and name
    *   GPU usage and name
    *   RAM usage
    *   Memory used by the process
*   **3D Camera:** A simple camera that can be moved and rotated.

## How to Compile and Run

1.  **Install Rust:** If you haven't already, install Rust from [rustup.rs](https://rustup.rs/).
2.  **Clone the repository:**
    ```bash
    git clone <REPOSITORY_URL>
    cd minecraft-rust
    ```
3.  **Run the application:**
    ```bash
    cargo run
    ```

## Dependencies

*   `wgpu`: For 3D graphics rendering.
*   `winit`: For windowing and event handling.
*   `egui`: For creating the debug overlay GUI.
*   `egui-wgpu`: For integrating `egui` with `wgpu`.
*   `egui-winit`: For integrating `egui` with `winit`.
*   `cgmath`: For vector and matrix math.
*   `sysinfo`: For getting system information (CPU, RAM, etc.).
*   `log` and `env_logger`: For logging messages to the console.
*   `tokio`: For the asynchronous runtime.

## Code Documentation

This project uses documentation comments in the code. To generate the documentation, run:

```bash
cargo doc --open
```

This will open the documentation in your web browser.