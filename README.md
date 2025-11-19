# James Park's Portfolio

A StarCraft-themed personal portfolio website built with **Rust** and **WebAssembly**. Features interactive StarCraft units (Zergling, Zealot, and Marine) chasing each other in the background, all powered by Rust compiled to WASM running in the browser.

## Tech Stack

- **Rust** - Core game logic and character AI
- **WebAssembly (WASM)** - Rust compiled to run in the browser
- **HTML5 Canvas** - Rendering the interactive characters
- **CSS3** - StarCraft-themed dark sci-fi styling
- **wasm-pack** - Building and packaging the Rust/WASM code

## Features

- Interactive StarCraft units with chase AI behavior
- Full-screen canvas animation background
- Responsive design for all screen sizes
- Built entirely with Rust for performance and reliability
- Automatic deployment via GitHub Actions

## Building Locally

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3 (for local server)

### Build Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/parkji30/parkji30.github.io.git
   cd parkji30.github.io
   ```

2. Build the WASM module:
   ```bash
   npm run build
   # or directly with wasm-pack:
   wasm-pack build --target web --out-dir pkg --release
   ```

3. Serve the site locally:
   ```bash
   npm run serve
   # or directly with Python:
   python3 -m http.server 8000
   ```

4. Open your browser to `http://localhost:8000`

## Development

For development with faster builds (debug mode):
```bash
npm run dev
```

To clean build artifacts:
```bash
npm run clean
```

## Project Structure

```
.
├── src/
│   └── lib.rs          # Rust game engine and character logic
├── pkg/                # Compiled WASM output (generated)
├── index.html          # Main page
├── styles.css          # StarCraft-themed styling
├── Cargo.toml          # Rust dependencies
└── package.json        # Build scripts
```

## Deployment

The site automatically deploys to GitHub Pages via GitHub Actions on every push to the main branch. The workflow:
1. Compiles Rust to WASM
2. Packages everything with wasm-pack
3. Deploys to the `gh-pages` branch

## License

MIT
