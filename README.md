# GameByAI - Wolfenstein by AI

A Wolfenstein-style game created with AI assistance using **Rust** and **macroquad**.

## 🦀 About

This project recreates the classic Wolfenstein 3D gameplay experience using modern Rust development with the macroquad game framework. The development process is AI-assisted, combining learning with practical game development.

## 🚀 Getting Started

### Prerequisites
- Rust (installed via rustup)
- Cargo (comes with Rust)

### Building and Running
```bash
# Clone the repository
git clone https://github.com/bonskari/GameByAI.git
cd GameByAI

# Build the project
cargo build

# Run the game
cargo run

# Run the visual test mode
cargo run -- visual-test
```

## 🎮 Game Features

- **Classic first-person shooter mechanics** (planned)
- **Retro-style graphics** inspired by Wolfenstein 3D
- **Modern Rust implementation** with memory safety
- **Cross-platform support** via macroquad
- **AI-assisted development** process

## 🛠️ Technologies Used

- **Rust** - Systems programming language
- **macroquad** - Simple and easy to use 2D/3D game framework
- **Cargo** - Rust package manager and build system

## 📁 Project Structure

```
├── src/
│   └── main.rs          # Main game code
├── cpp_backup/          # Previous C++ implementation
├── Cargo.toml          # Rust dependencies and metadata
├── .gitignore          # Git ignore patterns
└── README.md           # This file
```

## 🎯 Development Status

- ✅ Basic project setup
- ✅ Rust toolchain configuration  
- ✅ macroquad integration
- ✅ Basic game window with graphics
- 🔄 Game engine development (in progress)
- ⏳ Raycasting renderer (planned)
- ⏳ Player movement and controls (planned)
- ⏳ Level loading and rendering (planned)

## 🤝 Contributing

This is a learning project focused on AI-assisted game development. Feel free to explore the code and suggest improvements!

## 📝 License

[Add license information] 

## 🧪 Testing

The project includes an automated visual test system that validates textures and movement:

### Visual Test Mode
The visual test mode automatically walks through the level, testing textures and movement:
```bash
# Run the visual test
cargo run -- visual-test
```

The visual test will:
- Walk through the level continuously
- Validate textures at each position
- Generate screenshots for comparison
- Run indefinitely until manually stopped

To stop the test, press `Esc` or close the window. 