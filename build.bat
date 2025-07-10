@echo off
echo DuxNet - Decentralized P2P Platform
echo ====================================

echo.
echo Checking if Rust is installed...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed. Please install Rust first:
    echo 1. Visit https://rustup.rs/
    echo 2. Download and run rustup-init.exe
    echo 3. Restart your terminal
    echo 4. Run this script again
    pause
    exit /b 1
)

echo Rust is installed. Building DuxNet...
echo.

echo Building in release mode...
cargo build --release
if %errorlevel% neq 0 (
    echo Build failed. Please check the error messages above.
    pause
    exit /b 1
)

echo.
echo Build successful! 
echo.
echo To run the application:
echo 1. Open a new terminal
echo 2. Navigate to this directory
echo 3. Run: cargo run --release
echo.
echo The application will start:
echo - P2P node on port 8080
echo - Web API on port 8081  
echo - Web interface at http://localhost:8081
echo.
pause 