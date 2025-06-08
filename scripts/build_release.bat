@echo off
echo =====================================================
echo        WOLFENSTEIN BY AI - BUILD SCRIPT
echo =====================================================
echo.

REM Add Cargo to PATH for this session
set PATH=%PATH%;%USERPROFILE%\.cargo\bin

echo [1/4] Building release version...
cargo build --release
if errorlevel 1 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo [2/4] Creating distribution directories...
if not exist "build" mkdir build
if not exist "build\win64" mkdir build\win64

echo [3/4] Copying executable to distribution folder...
copy "target\release\wolfenstein-ai.exe" "build\win64\" >nul

echo [4/4] Distribution ready!
echo.
echo =====================================================
echo   BUILD COMPLETE - Ready for distribution!
echo =====================================================
echo.
echo Files available in: build\win64\
echo - wolfenstein-ai.exe (main game executable)
echo - README.txt (instructions)
echo.
echo To run: Just double-click wolfenstein-ai.exe
echo To share: Send the entire "build\win64" folder
echo.
pause 