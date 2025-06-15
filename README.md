# GameByAI - 3D Game Engine

A 3D first-person game created with AI assistance using **Rust** and **macroquad**.

## 🦀 About

This project creates a classic first-person 3D gameplay experience using modern Rust development with the macroquad game framework. The development process is AI-assisted, combining learning with practical game development.

**🆕 NEW: Complete ECS Implementation** - The game now features a fully functional Entity Component System with texture rendering, collision detection, and excellent performance (116-121 FPS).

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

# Run the visual test mode with AI pathfinding
cargo run -- visual-test
```

## 🎮 Game Features

- **✅ Full 3D first-person rendering** with textured walls, floors, and ceilings
- **✅ Complete ECS architecture** with 253 entities (52 walls, 100 floors, 100 ceilings)
- **✅ Texture-based rendering** with proper material lookup system
- **✅ ECS collision detection** working seamlessly with grid-based detection
- **✅ Hybrid Legacy/ECS system** with both architectures coexisting
- **✅ Excellent performance** maintaining 116-121 FPS consistently
- **✅ Advanced pathfinding system** with A* algorithm and visual debugging
- **✅ Modern 3D graphics** with procedural textures and lighting
- **✅ Integrated testing system** with automated bot navigation
- **✅ Real-time minimap** with pathfinding visualization
- **✅ First-person controls** (WASD movement, mouse look, jumping)
- **✅ Cross-platform support** via macroquad
- **✅ AI-assisted development** process

## 🏗️ ECS Architecture

The game features a **complete Entity Component System (ECS)** implementation:

### Core ECS Components
- **Transform** - Position, rotation, and scale for all entities
- **StaticRenderer** - Texture-based rendering with material types
- **Collider** - Physics-engine-style collision with shapes and materials
- **Player** - Player-specific data and settings
- **Wall/Floor/Ceiling** - Level geometry components with texture mapping
- **MaterialType** - Texture material system (Wall, Floor, Ceiling variants)

### ECS Architecture
- **Direct Component Queries** - Efficient direct world queries for game logic
- **Rendering System** - Handles texture lookup and 3D rendering for 253 entities
- **Collision Detection** - Grid-based collision using entity component queries
- **Physics Integration** - Gravity, jumping, and physics via direct component access

### Current ECS Implementation Status
- **✅ Player Entity**: Fully migrated to ECS with Transform + Player components
- **✅ 253 Static Entities**: 52 walls, 100 floors, 100 ceilings all ECS-based
- **✅ ECS Rendering System**: StaticRenderer actively rendering all ECS entities
- **✅ Texture System**: Complete material-based texture rendering via ECS
- **✅ ECS Collision Detection**: Grid-based collision working perfectly
- **✅ Performance**: Excellent 116-121 FPS with full ECS rendering
- **✅ Hybrid System**: Legacy and ECS coexisting seamlessly

## 🎨 Texture System

The game features a complete texture loading and rendering system:

### Loaded Textures
- **tech_panel.png** - Wall textures
- **hull_plating.png** - Wall textures  
- **control_system.png** - Wall textures
- **energy_conduit.png** - Wall textures
- **floor.png** - Floor textures with linear filtering
- **ceiling.png** - Ceiling textures

### Material System
- **MaterialType enum** with Wall, Floor, Ceiling variants
- **Texture lookup** by material type
- **Proper UV mapping** for all surfaces
- **Linear filtering** for smooth texture rendering

## 🧠 AI Pathfinding System

The game features an advanced AI pathfinding system with:
- **A* pathfinding algorithm** for optimal route finding
- **Wall detection and avoidance** 
- **Visual debugging on minimap** showing:
  - Blue areas: A* explored nodes (search area)
  - Red areas: Actual pathfinding routes
  - Yellow circle: Current target waypoint
  - Green dot: Player position and direction

## 🛠️ Technologies Used

- **Rust** - Systems programming language
- **macroquad** - Simple and easy to use 2D/3D game framework
- **Custom ECS** - Complete Entity Component System implementation
- **Texture Loading** - PNG texture support with material system
- **Grid-based Collision** - Efficient spatial collision detection
- **Cargo** - Rust package manager and build system
- **clap** - Command line argument parsing

## 📁 Project Structure

```
├── src/
│   ├── main.rs              # Main entry point and CLI
│   ├── cli.rs               # Command line interface
│   ├── game/
│   │   ├── mod.rs           # Game module
│   │   ├── state.rs         # Game state management
│   │   ├── player.rs        # Player mechanics
│   │   ├── map.rs           # Level data and rendering
│   │   ├── input.rs         # Centralized input handling
│   │   ├── ecs_state.rs     # ECS game state
│   │   └── rendering/       # 3D graphics engine
│   ├── ecs/                 # Entity Component System
│   │   ├── mod.rs           # ECS module exports
│   │   ├── entity.rs        # Entity management
│   │   ├── component.rs     # Component storage
│   │   ├── world.rs         # ECS world container
│   │   ├── system.rs        # System management
│   │   ├── components.rs    # Game-specific components
│   │   ├── query.rs         # Query system (future)
│   │   └── resource.rs      # Resource management
│   └── testing/
│       ├── mod.rs           # Testing module
│       ├── visual_tests.rs  # AI bot and pathfinding
│       ├── tests.rs         # Unit tests
│       ├── runner.rs        # Test runner
│       └── screenshot_validator.rs # Visual validation
├── cpp_backup/              # Previous C++ implementation
├── Cargo.toml              # Rust dependencies and metadata
├── .gitignore              # Git ignore patterns
└── README.md               # This file
```

## 🎯 Development Status

- ✅ Basic project setup
- ✅ Rust toolchain configuration  
- ✅ macroquad integration
- ✅ Advanced 3D graphics engine with procedural textures
- ✅ Complete player movement system (WASD, mouse look, jumping)
- ✅ Level loading and rendering system
- ✅ AI pathfinding with A* algorithm
- ✅ Visual debugging and testing system
- ✅ Minimap with real-time pathfinding visualization
- ✅ Automated testing with AI bot navigation
- ✅ **Entity Component System (ECS) implementation**
- ✅ **Hybrid architecture with runtime switching**
- ✅ **Centralized input system**
- ✅ **Full feature parity between Legacy and ECS systems**

## 🎮 Controls

- **WASD** - Move and strafe
- **Mouse** - Look around (FPS-style)
- **Space** - Jump
- **Tab** - Toggle between 3D and 2D view
- **M** - Toggle mouse capture
- **E** - **Switch between Legacy ↔ ECS systems** (NEW!)
- **Esc** - Exit game

## 🧪 Testing

The project includes an advanced automated visual test system with AI pathfinding:

### Visual Test Mode
The visual test mode features an AI bot that automatically navigates through the level:
```bash
# Run the visual test with AI pathfinding
cargo run -- visual-test

# Run with custom duration (default: 15 seconds)
cargo run -- visual-test --duration 30
```

The visual test will:
- **Generate optimal paths** using A* pathfinding algorithm
- **Navigate through 64+ waypoints** automatically
- **Visualize pathfinding** on the minimap in real-time
- **Test wall collision detection** and avoidance
- **Display progress** with overlay information
- **Complete automatically** after the specified duration

### What You'll See:
- **Main 3D View**: Full first-person 3D rendering
- **Minimap (top-right)**: 
  - Blue areas: A* algorithm exploration
  - Red areas: Calculated pathfinding routes
  - Yellow circle: Current target waypoint
  - Green dot: AI bot position and direction
- **Overlay (top-left)**: Progress information and test status
- **System Indicator**: Shows whether Legacy or ECS system is active

## 🏛️ Architecture Comparison

| Feature | Legacy System | ECS System |
|---------|---------------|------------|
| **Performance** | 120+ FPS | 120+ FPS |
| **Movement** | ✅ WASD + Mouse | ✅ WASD + Mouse |
| **Jumping** | ✅ Physics + Gravity | ✅ Physics + Gravity |
| **Collision** | ✅ Wall Detection | ✅ Wall Detection |
| **Code Structure** | Monolithic | Component-based |
| **Extensibility** | Limited | High |
| **Memory Usage** | Lower | Slightly Higher |
| **Runtime Switch** | N/A | ✅ Press 'E' |

## 🤝 Contributing

This is a learning project focused on AI-assisted game development. Feel free to explore the code and suggest improvements!

## 📝 License

[Add license information] 