@echo off
echo Starting DuxNet - Decentralized P2P Platform
echo ============================================

echo.
echo Checking if the application is built...
if not exist "target\release\duxnet.exe" (
    echo Application not built. Running build first...
    call build.bat
    if %errorlevel% neq 0 (
        echo Build failed. Cannot run application.
        pause
        exit /b 1
    )
)

echo.
echo Starting DuxNet node...
echo.
echo The application will start:
echo - P2P node on port 8080
echo - Web API on port 8081
echo - Web interface at http://localhost:8081
echo.
echo Press Ctrl+C to stop the application
echo.

cargo run --release

echo.
echo Application stopped.
pause 