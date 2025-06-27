@echo off

echo Starting debug...
echo Testing Rust detection...

cargo --version >nul 2>&1

echo After cargo check...
echo Error level is: %errorlevel%

if errorlevel 1 (
    echo INSIDE IF BLOCK - Rust not found
    echo Can you see this line?
    echo And this one too?
    set /p "TEST=Debug question - type anything: "
) else (
    echo Rust is installed
)

echo Debug complete
pause 