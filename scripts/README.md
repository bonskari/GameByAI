# Wolfenstein by AI - Scripts & Testing

## Integrated Testing System

The game now includes a built-in testing system! No external scripts needed.

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

### Available Tests

1. **Graphics Initialization** - Tests basic macroquad graphics setup
2. **Game Loop** - Tests game loop performance and stability (3-second test)
3. **Player Movement** - Tests player movement logic and physics
4. **Collision Detection** - Tests wall collision and boundary checking

### Test Features

- ✅ **Cross-platform** - Works on Windows, macOS, Linux
- ✅ **No external dependencies** - Built into the game executable  
- ✅ **Automated reporting** - Clear pass/fail results with timing
- ✅ **Verbose mode** - Detailed diagnostic output
- ✅ **Exit codes** - Returns 0 for success, 1 for failures
- ✅ **Performance metrics** - FPS, timing, and system info

### Command Line Options

```
Usage: wolfenstein-ai [COMMAND]

Commands:
  test  Run automated tests
  help  Print this message or the help of the given subcommand(s)

Test Options:
  <TEST_TYPE>         Specific test to run (all, graphics, movement, collision) [default: all]
  -t, --timeout <TIMEOUT>  Timeout in seconds for each test [default: 10]
  -v, --verbose       Verbose output
  -h, --help          Print help
```

### Example Output

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

## Development Commands

```bash
# Development mode (compile + run)
cargo run

# Testing mode  
cargo run -- test all --verbose

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