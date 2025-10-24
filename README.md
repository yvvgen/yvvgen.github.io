## Resume

A modern portfolio/resume site built with Rust and WebAssembly.

## 🛠️ Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Yew](https://yew.rs/)** - Rust framework for building web apps with WebAssembly
- **[Trunk](https://trunkrs.dev/)** - WASM web application bundler
- **[Tailwind CSS](https://tailwindcss.com/)** - Utility-first CSS framework

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk

# Install Tailwind
npm install
```

### Development

```bash
# Run dev server with hot reload
trunk serve

# Open http://localhost:8080
```

### Build

```bash
# Build for production
trunk build --release

# Output in dist/
```

## 📦 Deployment

```bash
# Deploy to GitHub Pages (yvvgen.github.io)
./deploy.sh
```

The script will:
1. Commit and push changes from `dev` → `main`
2. Build the production bundle
3. Deploy to `gh-branch` for GitHub Pages

## 📁 Project Structure

```
├── src/           # Rust source code
├── styles/        # Tailwind CSS
├── dist/          # Build output (generated)
├── Trunk.toml     # Build configuration
└── deploy.sh      # Deployment script
```

## 🌐 Live Site

[https://yvvgen.github.io](https://yvvgen.github.io) yvvgen.github.io
