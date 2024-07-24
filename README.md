# CS2 Internal Cheat Base ![CS2](https://img.shields.io/badge/game-CS2-blue) ![LIC](https://img.shields.io/github/license/W1lliam1337/digital-sdk) ![LANG](https://img.shields.io/badge/language-rust-orange)
![App menu](https://i.imgur.com/LBKe37u.png)

## Overview
CS2 Internal Cheat Base is a Rust-based internal cheat framework for Counter-Strike 2 (CS2). Designed with modularity and performance in mind, this project provides a solid foundation for developing various cheat features. The framework supports dynamic interface access and module management, making it an ideal starting point for internal cheat development.

## Features
- **Rust Implementation:** Utilizes Rust for its performance and safety features, ensuring a reliable codebase.
- **Modular Design:** Organizes game interfaces and components into separate modules for easy management and extension.
- **Interface Handling:** Automates the creation and access of game interfaces with macros, reducing boilerplate code.
- **Pattern Searching:** Implements dynamic pattern searching for locating functions and data structures in memory.
- **DirectX11 Integration**: Built-in support for rendering overlays using DirectX11.
- **MinHook Integration**: Uses the MinHook library for function hooking.

## Getting Started
- **Clone the repository**:
```
git clone https://github.com/W1lliam1337/cstrike2-hack.git
cd cstrike2-hack
```
- **Install Rust**: Ensure you have Rust installed. If not, install it from rust-lang.org.
- **Build the project:**
```
cargo build --release
```
- **Run the project**:
Inject the compiled binary into the CS2 process using your preferred DLL injector.

## Usage:
- The in-game menu can be toggled with the `Insert` key.
- Extend the project by implementing additional features using the provided hooks and patterns.

## Contributions:
We welcome contributions! Feel free to open issues or submit pull requests for improvements and new features.

## Contact:
For any inquiries, contact https://en1gma.tech/ & https://t.me/animstate.

## License
This project is licensed under the **MIT License**. See the LICENSE file for more details.
