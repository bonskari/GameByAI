@echo off
echo 🔆 Starting comprehensive visual tests (lighting + navigation)...
echo.
echo This will run the complete visual test suite including:
echo - Lighting performance tests (progressive light count testing)
echo - AI bot navigation with A* pathfinding
echo - Performance measurement and analysis
echo.

REM Run complete visual test suite (30 second bot navigation duration)
cargo run --release -- visual-test --duration 30

echo.
echo ✅ All visual tests completed!
echo Check console output above for performance results.
pause 