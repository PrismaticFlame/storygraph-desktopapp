# StoryGraph Desktop App

A lightweight desktop wrapper for [StoryGraph](https://app.thestorygraph.com).
Sits in your system tray for quick access without opening a browser.

Not affliated with the official TheStoryGraph.

## Download

Go to the [Releases page](https://github.com/PrismaticFlame/storygraph-desktopapp/releases) and download the file for your OS:

| OS | File to download |
| --- | --- |
| Windows | `.msi` or `-setup.exe` |
| Linux (Debian/Ubuntu/Arch) | `.deb` |
| Linux (other) | `.AppImage` |

## For Developers

Prerequisites: [Rust](https://rustup.rs) and the 
[Tauri CLI](https://tauri.app/start/prerequisites/)
```bash
git clone https://github.com/you/storygraph-app
cd storygraph-app
cargo tauri dev
```
