# Prerequisites

## Rust (Nightly)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Switch to nightly
rustup default nightly
```

## Leptos

```bash
# Install cargo-leptos
cargo install cargo-leptos

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Tauri

```bash
# Install Tauri CLI
cargo install tauri-cli

# macOS dependencies (if needed)
xcode-select --install
```

For other platforms, see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/).

## Just

```bash
# Install just
cargo install just
```

## PostgreSQL

**macOS (Homebrew)**
```bash
brew install postgresql@17
brew services start postgresql@17
```

**Windows**

Download and install from [postgresql.org/download/windows](https://www.postgresql.org/download/windows/).

**Linux (Debian/Ubuntu)**
```bash
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

The database itself is created via `just reset_db` (see README).

## pnpm

```bash
# Install pnpm
npm install -g pnpm
```
