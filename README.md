# GameByAI - 3D Game Engine

A 3D first-person game created with AI assistance using **Rust** and **macroquad**.

## 🦀 About

This project creates a classic first-person 3D gameplay experience using modern Rust development with the macroquad game framework. The development process is AI-assisted, combining learning with practical game development.

**🆕 NEW: Hybrid ECS A* Pathfinding System** - The game now features a complete Entity Component System with intelligent A* pathfinding, reusable components, and excellent performance (120+ FPS after lighting optimization).

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

# Run visual test with custom duration
cargo run -- visual-test -d 30
```

## 🎮 Game Features

- **✅ Full 3D first-person rendering** with textured walls, floors, and ceilings
- **✅ Complete ECS architecture** with 253 entities (52 walls, 100 floors, 100 ceilings)
- **✅ Hybrid ECS A* pathfinding system** with reusable Pathfinder component
- **✅ Intelligent TestBot navigation** using A* algorithm for optimal pathfinding
- **✅ Texture-based rendering** with proper material lookup system
- **✅ ECS collision detection** working seamlessly with grid-based detection
- **✅ Excellent performance** maintaining 120+ FPS consistently (after lighting optimization)
- **✅ Modern 3D graphics** with procedural textures ⚠️ (dynamic lighting disabled for performance)
- **✅ Integrated testing system** with automated bot navigation
- **✅ Real-time minimap** with pathfinding visualization
- **✅ First-person controls** (WASD movement, mouse look, jumping)
- **✅ Cross-platform support** via macroquad
- **✅ AI-assisted development** process

## 🏗️ ECS Architecture

The game features a **complete Entity Component System (ECS)** implementation with hybrid pathfinding:

### Core ECS Components
- **Transform** - Position, rotation, and scale for all entities
- **StaticRenderer** - Texture-based rendering with material types
- **Collider** - Physics-engine-style collision with shapes and materials
- **Player** - Player-specific data and settings
- **Wall/Floor/Ceiling** - Level geometry components with texture mapping
- **MaterialType** - Texture material system (Wall, Floor, Ceiling variants)
- **🆕 Pathfinder** - Reusable A* pathfinding component for any entity
- **🆕 TestBot** - Automated testing bot with waypoint navigation

### ECS Systems
- **Rendering System** - Handles texture lookup and 3D rendering for 253 entities
- **Collision Detection** - Grid-based collision using entity component queries
- **🆕 PathfindingSystem** - Processes all entities with Pathfinder components
- **Physics Integration** - Gravity, jumping, and physics via direct component access

### Hybrid Pathfinding Model
- **TestBot Component**: Manages high-level behavior (waypoints, test duration)
- **Pathfinder Component**: Handles low-level pathfinding (A* algorithm, path following)
- **Reusable Design**: Any entity can add a Pathfinder component for intelligent navigation
- **A* Algorithm**: Proper heuristic-based pathfinding with obstacle avoidance
- **Performance Optimized**: Binary heap implementation for efficient pathfinding

### Current ECS Implementation Status
- **✅ Player Entity**: Fully migrated to ECS with Transform + Player components
- **✅ 253 Static Entities**: 52 walls, 100 floors, 100 ceilings all ECS-based
- **✅ ECS Rendering System**: StaticRenderer actively rendering all ECS entities
- **✅ Texture System**: Complete material-based texture rendering via ECS
- **✅ ECS Collision Detection**: Grid-based collision working perfectly
- **✅ 🆕 Hybrid Pathfinding**: TestBot + Pathfinder components working together
- **✅ Performance**: Excellent 120+ FPS with full ECS rendering and pathfinding (lighting system disabled)
- **⚠️ Technical Debt**: Dynamic lighting system exists but disabled for performance reasons

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

The game features an advanced hybrid ECS pathfinding system:

### A* Pathfinding Algorithm
- **Heuristic-based pathfinding** for optimal route calculation
- **Binary heap optimization** for efficient node processing
- **Obstacle avoidance** with proper wall detection
- **Grid-based navigation** integrated with map system
- **Diagonal movement support** with proper cost calculation

### Pathfinder Component (Reusable)
- **Target position tracking** for navigation goals
- **Path calculation and storage** for smooth movement
- **Movement and rotation speeds** configurable per entity
- **Path following logic** with waypoint progression
- **Stuck detection and recovery** for robust navigation

### Visual Debugging
- **Real-time minimap visualization** showing:
  - Blue areas: A* explored nodes (search area)
  - Red areas: Actual pathfinding routes
  - Yellow circle: Current target waypoint
  - Green dot: Player/bot position and direction

## 🛠️ Technologies Used

- **Rust** - Systems programming language
- **macroquad** - Simple and easy to use 2D/3D game framework
- **Custom ECS** - Complete Entity Component System implementation
- **A* Pathfinding** - Intelligent navigation with binary heap optimization
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
│   │   ├── systems.rs       # Game-specific systems
│   │   ├── pathfinding.rs   # 🆕 A* pathfinding algorithms
│   │   ├── query.rs         # Query system
│   │   └── resource.rs      # Resource management
│   └── testing/
│       ├── mod.rs           # Testing module
│       ├── tests.rs         # Unit tests
│       ├── runner.rs        # Test runner
│       └── screenshot_validator.rs # Visual validation
├── cpp_backup/              # Previous C++ implementation
├── scripts/                 # Build and utility scripts
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
- ✅ **🆕 Hybrid ECS A* pathfinding system**
- ✅ **🆕 Reusable Pathfinder component for any entity**
- ✅ **🆕 Intelligent TestBot with A* navigation**
- ✅ Visual debugging and testing system
- ✅ Minimap with real-time pathfinding visualization
- ✅ Automated testing with AI bot navigation
- ✅ **Entity Component System (ECS) implementation**
- ✅ **Centralized input system**
- ✅ **Full ECS architecture with pathfinding integration**

## 🎮 Controls

- **WASD** - Move and strafe
- **Mouse** - Look around (FPS-style)
- **Space** - Jump
- **Tab** - Toggle between 3D and 2D view
- **M** - Toggle mouse capture
- **Esc** - Exit game

## 🧪 Testing

The project includes an advanced automated visual test system with AI pathfinding:

### Visual Test Mode
The visual test mode runs comprehensive testing including lighting performance and AI bot navigation:
```bash
# Run the complete visual test suite (lighting + bot navigation)
cargo run -- visual-test

# Run with custom bot navigation duration (default: 15 seconds)
cargo run -- visual-test --duration 30
cargo run -- visual-test -d 30
```

The visual test will:
- **🔆 Test lighting performance first** with progressive light count testing
- **Generate optimal paths** using A* pathfinding algorithm
- **Navigate through waypoints** automatically with intelligent pathfinding
- **Visualize pathfinding** on the minimap in real-time
- **Test wall collision detection** and avoidance
- **Display progress** with overlay information
- **Complete automatically** after the specified duration

### Lighting Performance Tests (Always Included)
Every visual test run includes a progressive lighting test sequence:
- **Baseline Test**: Performance with no lights (3 seconds)
- **Single Light Test**: Performance with 1 light (2 seconds)
- **Multiple Lights**: Performance with 8 strategically placed lights (3 seconds)
- **Many Lights**: Performance with 50 random lights (4 seconds)
- **Stress Test**: Performance with 100+ lights if previous tests show good performance (3 seconds)

This helps measure the impact of the lighting system on performance and identify optimal light counts for smooth gameplay.

### What You'll See:
- **Main 3D View**: Full first-person 3D rendering
- **Minimap (top-right)**: 
  - Blue areas: A* algorithm exploration
  - Red areas: Calculated pathfinding routes
  - Yellow circle: Current target waypoint
  - Green dot: AI bot position and direction
- **Overlay (top-left)**: Progress information and test status
- **Console Output**: Real-time pathfinding calculations and navigation updates

## 🏛️ Architecture Highlights

### ECS + A* Pathfinding Integration
- **Modular Design**: Pathfinding is a reusable component system
- **High Performance**: 120+ FPS with full pathfinding calculations
- **Scalable**: Any number of entities can use pathfinding simultaneously
- **Intelligent Navigation**: Proper obstacle avoidance and optimal routing

### Component Separation
- **TestBot**: High-level waypoint management and test behavior
- **Pathfinder**: Low-level A* pathfinding and movement execution
- **Transform**: Position and rotation data
- **Clean Architecture**: Each component has a single responsibility

## 🤝 Contributing

This is a learning project focused on AI-assisted game development. Feel free to explore the code and suggest improvements!

## Try these techniques

https://diglib.eg.org/items/93fc78c0-71fa-4511-8564-a7e5268bf27a
