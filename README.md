# Goard

Goard (pronounced "guard") is a modern, blazing-fast dashboard application built with Rust, powered by Eframe and Egui libraries. This elegantly structured MVC project delivers stunning data visualization through an interactive dashboard and Gantt chart interface.

## Key Features

- Real-time job monitoring
- Interactive dashboard view
- Gantt chart visualization
- Period-based job filtering
- Secure SSH-based data retrieval from HPC clusters
- Loading state indicators
- Responsive UI with resizable components

## Project Architecture

```
src/
├── main.rs         # Entry point & application bootstrap
├── app.rs          # Core application state management
├── models/         # Data structures & business logic
├── views/          # UI components & layouts
```

## Getting Started

### Prerequisites

- Rust and Cargo installed
- SSH access to HPC cluster
- Git

### Quick Start

#### Testing Locally

1. Set up SSH access to your HPC cluster (default configuration: "grenoble.g5k").

2. Ensure you have the latest stable Rust:
    ```bash
    rustup update
    ```

3. Launch the application:
    ```bash
    cargo run --release
    ```

#### Web Development

Build and run as a web application using WebAssembly:

1. Add WASM target:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```

2. Install Trunk:
    ```bash
    cargo install --locked trunk
    ```

3. Serve locally:
    ```bash
    trunk serve
    ```
    Access at `http://127.0.0.1:8080/index.html#dev`

> Append `#dev` to skip PWA caching during development

#### Web Deployment

1. Build for production:
    ```bash
    trunk build --release
    ```

2. Deploy the generated `dist` directory to your preferred hosting platform

> The app supports offline functionality through service worker caching!


## Contributing

We welcome contributions! Here's how you can help:

- Report bugs
- Propose features
- Submit PRs

## License

This project is open source and available under the LGPL-2.1 license.

## Support

Star this repo if you find it helpful!

