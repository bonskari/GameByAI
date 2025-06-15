# Wolfenstein by AI - Scripts & Testing

## Integrated Testing System

The game now includes a built-in testing system with advanced A* pathfinding! No external scripts needed.

### Usage

**Run the game normally:**
```bash
cargo run
```

**Run automated tests:**
```bash
# Run all tests
cargo run -- test

# Run specific test
cargo run -- test movement
cargo run -- test collision
cargo run -- test graphics

# Run with verbose output and custom timeout
cargo run -- test all --verbose --timeout 15
```

**🆕 Run visual tests with A* pathfinding:**
```bash
# Run visual test with intelligent AI bot navigation
cargo run -- visual-test

# Run with custom duration (default: 15 seconds)
cargo run -- visual-test --duration 30
cargo run -- visual-test -d 30
```

### Available Tests

1. **Graphics Initialization** - Tests basic macroquad graphics setup
2. **Game Loop** - Tests game loop performance and stability (3-second test)
3. **Player Movement** - Tests player movement logic and physics
4. **Collision Detection** - Tests wall collision and boundary checking
5. **🆕 Visual Test with A* Pathfinding** - Advanced AI bot navigation testing

### 🆕 Visual Test Features

- ✅ **Intelligent A* Pathfinding** - Bot uses optimal pathfinding algorithms
- ✅ **Real-time Pathfinding Visualization** - See A* algorithm in action
- ✅ **Hybrid ECS Architecture** - TestBot + Pathfinder components
- ✅ **Waypoint Navigation** - Intelligent navigation between waypoints
- ✅ **Performance Monitoring** - 120+ FPS with full pathfinding
- ✅ **Visual Debugging** - Minimap shows pathfinding calculations
- ✅ **Console Output** - Real-time pathfinding status and navigation updates

### Visual Test Display

When running `cargo run -- visual-test`, you'll see:

- **Main 3D View**: Full first-person 3D rendering
- **Minimap (top-right)**: 
  - Blue areas: A* algorithm exploration
  - Red areas: Calculated pathfinding routes
  - Yellow circle: Current target waypoint
  - Green dot: AI bot position and direction
- **Overlay (top-left)**: Progress information and test status
- **Console Output**: Real-time pathfinding calculations

### Test Features

- ✅ **Cross-platform** - Works on Windows, macOS, Linux
- ✅ **No external dependencies** - Built into the game executable  
- ✅ **Automated reporting** - Clear pass/fail results with timing
- ✅ **Verbose mode** - Detailed diagnostic output
- ✅ **Exit codes** - Returns 0 for success, 1 for failures
- ✅ **Performance metrics** - FPS, timing, and system info
- ✅ **🆕 A* Pathfinding** - Advanced AI navigation testing

### Command Line Options

```
Usage: wolfenstein-ai [COMMAND]

Commands:
  test         Run automated tests
  visual-test  Run visual test with A* pathfinding bot
  help         Print this message or the help of the given subcommand(s)

Test Options:
  <TEST_TYPE>         Specific test to run (all, graphics, movement, collision) [default: all]
  -t, --timeout <TIMEOUT>  Timeout in seconds for each test [default: 10]
  -v, --verbose       Verbose output
  -h, --help          Print help

Visual Test Options:
  -d, --duration <DURATION>  Test duration in seconds [default: 15]
  -h, --help                 Print help
```

### Example Output

**Standard Tests:**
```
=== WOLFENSTEIN BY AI - INTEGRATED TEST SYSTEM ===
Timeout: 10s per test | Verbose: true
Platform: windows | Graphics: macroquad

✓ PASS Graphics Initialization - Graphics OK (800x600) (0.02s)
✓ PASS Game Loop - Game loop OK (60 FPS) (3.01s)  
✓ PASS Player Movement - Movement OK (pos: 5.03,5.00, rot: 0.02) (0.00s)
✓ PASS Collision Detection - Collision detection OK (0.00s)

=== TEST SUMMARY ===
Results: 4/4 tests passed
Success rate: 100.0%
```

**🆕 Visual Test with A* Pathfinding:**
```
🎮 Starting visual test mode with A* pathfinding...
🤖 TestBot attached with A* pathfinding capabilities
🗺️ A* pathfinding: (3,1) → (2,1) | Path found: [(3,1), (2,1)] | 2 nodes explored
🤖 TestBot moving to waypoint 1: (2.5, 1.5) - East corridor
🗺️ A* pathfinding: (2,1) → (3,1) | Path found: [(2,1), (3,1)] | 2 nodes explored
🤖 TestBot moving to waypoint 2: (3.5, 1.5) - Continue east
...
🎯 Visual test completed successfully after 15.0 seconds
```

## ECS Architecture Testing

The testing system now validates the complete ECS architecture:

### ECS Components Tested
- **Transform** - Position, rotation, scale validation
- **StaticRenderer** - Texture rendering verification
- **Collider** - Physics collision testing
- **Player** - Player entity functionality
- **🆕 Pathfinder** - A* pathfinding component testing
- **🆕 TestBot** - Automated navigation testing

### ECS Systems Tested
- **Rendering System** - 253 entities rendered at 120+ FPS
- **Collision System** - Grid-based collision detection
- **🆕 PathfindingSystem** - A* algorithm processing
- **Physics System** - Movement and gravity

## Development Commands

```bash
# Development mode (compile + run)
cargo run

# Standard testing mode  
cargo run -- test all --verbose

# 🆕 Visual testing with A* pathfinding
cargo run -- visual-test -d 30

# Release build (for distribution)
cargo build --release

# Check for errors/warnings
cargo check
```

## Future Platform Scripts

When needed, platform-specific build scripts will be added here:
- `build.sh` - Unix build script
- `build.bat` - Windows build script  
- `package.sh` - Distribution packaging
- `ci.yml` - GitHub Actions configuration

## 🆕 Pathfinding System Testing

The visual test mode now includes comprehensive pathfinding validation:

### A* Algorithm Testing
- **Heuristic calculation** - Manhattan distance heuristics
- **Binary heap optimization** - Efficient node processing
- **Obstacle avoidance** - Wall detection and routing
- **Path optimization** - Shortest path calculation
- **Performance validation** - Real-time pathfinding at 120+ FPS

### Component Integration Testing
- **TestBot + Pathfinder** - Hybrid component model
- **ECS query system** - Component retrieval and updates
- **System processing** - PathfindingSystem execution
- **Memory efficiency** - Component storage optimization

This testing framework ensures the hybrid ECS A* pathfinding system works correctly across all platforms and provides a foundation for future AI entity development. 