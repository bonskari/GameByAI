# Testing Protocol Rule

## Always Test Changes with run_test.bat

**CRITICAL RULE**: After making ANY code changes to this Rust game project, you MUST test using the provided batch file:

```bash
.\run_test.bat
```

## Why This Rule Exists

1. **Proper Testing Environment**: The `run_test.bat` file is specifically designed for quick validation of game functionality
2. **Consistent Testing**: Ensures all testing is done the same way across different development sessions  
3. **Catches Runtime Issues**: Compilation success doesn't guarantee the game actually runs correctly
4. **Movement/Collision Validation**: This game has complex ECS collision detection that needs runtime validation
5. **Performance Validation**: The test runs long enough to catch performance regressions

## Testing Workflow

1. Make code changes
2. Run `cargo check` (optional, for quick compile check)
3. **ALWAYS** run `.\run_test.bat` 
4. Wait for game to start and verify:
   - Game loads without crashes
   - Movement works (WASD keys)
   - Collision detection works (can't walk through walls)
   - Rendering works (can see the 3D environment)
   - Performance is acceptable (FPS display)

## Never Skip This Step

- Don't assume compilation success means the game works
- Don't skip testing "small" changes - they can break runtime behavior
- Don't use `cargo run` directly - use the standardized test script

This rule was established after discovering that ECS refactoring broke collision detection despite successful compilation. 