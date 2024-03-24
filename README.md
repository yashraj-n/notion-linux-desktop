# Notion Linux Desktop Build
This repository contains the code to build the unofficial Notion.so Linux desktop app. The code is based on the [Notion.so web client](https://www.notion.so/). Built using [Tauri](https://tauri.app) and Rust.

## Installation

Get a .deb or .AppImage release from [Releases](https://github.com/yashraj-n/notion-linux-desktop/releases/tag/v0.1.0)

- For Debian/Ubuntu

> Its recommended to use the *.deb* release package

```bash
# assuming you have the .deb file in current directory

# Install using dpkg

sudo dpkg -i <filename>.deb
```

- For other Linux distributions

> Its recommended to use the *.AppImage* release package

```bash
# assuming you have the .AppImage file in current directory

# Make the AppImage executable

chmod +x <filename>.AppImage

# Run the AppImage

./<filename>.AppImage
```

## Development

### Prerequisites

- Rust
- Tauri CLI

### Steps

1. Clone the repository

```bash
git clone https://github.com/yashraj-n/notion-linux-desktop
```

2. Change directory

```bash
cd notion-linux-desktop
```

3. Install Tauri CLI

```bash
cargo install tauri-cli
```

4. Install Cargo dependencies

```bash
cargo tauri dev
```

> Wait until you get `Warn Waiting for your frontend dev server to start on http://localhost:8080/...`

5. Run the app

```bash
# Change directory into src-tauri
cd src-tauri
# Run the app

cargo run
```

## License

[MIT](LICENSE)
