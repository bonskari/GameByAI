@echo off

echo ================================
echo   GameByAI - Main Launcher
echo ================================
echo.

REM Check if Rust/Cargo is installed
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo [ERROR] Rust is not installed!
    echo.
    echo This game requires Rust to compile and run.
    echo.
    set /p "INSTALL_RUST=Do you want me to install Rust automatically? (Y/N): "
    
    if /i "%INSTALL_RUST%"=="Y" (
        echo.
        echo Installing Rust via winget...
        winget install Rustlang.Rustup
        echo.
        echo Installation complete! Please restart your terminal and run this script again.
        pause
        exit /b 1
    ) else (
        echo.
        echo Please install Rust manually:
        echo 1. Visit: https://rustup.rs/
        echo 2. Download and run rustup-init.exe
        echo 3. Restart your terminal
        echo 4. Run this script again
        pause
        exit /b 1
    )
)

echo [SUCCESS] Rust found!
cargo --version
echo.

REM Parse command line arguments
set "COMMAND=%1"

REM Default to normal run if no command specified
if "%COMMAND%"=="" set "COMMAND=run"

echo [INFO] Executing: %COMMAND%
echo.

REM Handle different commands
if "%COMMAND%"=="run" (
    echo Compiling and running GameByAI...
    cargo run
) else if "%COMMAND%"=="test" (
    echo Running AI pathfinding visual test...
    if "%2"=="" (
        cargo run -- visual-test
    ) else (
        cargo run -- visual-test --duration %2
    )
) else if "%COMMAND%"=="release" (
    echo Compiling and running optimized release build...
    cargo run --release
) else if "%COMMAND%"=="build" (
    echo Building release version...
    cargo build --release
) else if "%COMMAND%"=="check" (
    echo Checking code for errors...
    cargo check
) else if "%COMMAND%"=="clean" (
    echo Cleaning build artifacts...
    cargo clean
) else if "%COMMAND%"=="textures" (
    echo Generating AI textures...
    if "%2"=="" (
        cargo run -- generate-textures
    ) else (
        cargo run -- generate-textures --texture-type %2
    )
) else (
    echo [ERROR] Unknown command: %COMMAND%
    echo.
    echo Available commands:
    echo   run          - Normal development run
    echo   test [time]  - Visual test mode (default 15s)
    echo   release      - Optimized release run
    echo   build        - Build release executable
    echo   check        - Check code for errors
    echo   clean        - Clean build files
    echo   textures [type] - Generate AI textures
    echo.
    pause
    exit /b 1
)

echo.
echo ================================
echo   Execution finished
echo ================================
pause 