# DuxNet Installation Guide for Windows

## Prerequisites

### 1. Install Rust

1. Visit [https://rustup.rs/](https://rustup.rs/)
2. Download `rustup-init.exe`
3. Run the installer and follow the prompts
4. **Important**: Restart your terminal/command prompt after installation

### 2. Verify Installation

Open a new command prompt and run:
```cmd
rustc --version
cargo --version
```

You should see version numbers for both commands.

## Building and Running DuxNet

### Option 1: Using Batch Files (Recommended)

1. **Build the application**:
   ```cmd
   build.bat
   ```

2. **Run the application**:
   ```cmd
   run.bat
   ```

### Option 2: Using Command Line

1. **Build the application**:
   ```cmd
   cargo build --release
   ```

2. **Run the application**:
   ```cmd
   cargo run --release
   ```

## What Happens When You Start DuxNet

When you run the application, it will:

1. **Start a P2P node** on port 8080
2. **Start a web API server** on port 8081
3. **Open a web interface** at http://localhost:8081

## Using the Web Interface

1. Open your web browser
2. Navigate to http://localhost:8081
3. You'll see the DuxNet dashboard with:
   - Service registration
   - Service discovery
   - Task submission
   - Escrow management
   - Network statistics

## Troubleshooting

### "cargo is not recognized"
- Make sure you've installed Rust using rustup
- Restart your command prompt after installation
- Try running `rustup update` to ensure you have the latest version

### Build Errors
- Make sure you have a stable internet connection (for downloading dependencies)
- Try running `cargo clean` and then `cargo build --release`
- Check that you're in the correct directory (should contain `Cargo.toml`)

### Port Already in Use
- If port 8080 or 8081 is already in use, you can modify the ports in `src/main.rs`
- Look for the lines with `8080` and `8081` and change them to available ports

### Firewall Issues
- Windows Firewall may block the application
- Allow the application through the firewall when prompted
- Or manually add an exception for the application

## Development

### Running in Development Mode
```cmd
cargo run
```

### Running Tests
```cmd
cargo test
```

### Code Formatting
```cmd
cargo fmt
```

### Linting
```cmd
cargo clippy
```

## Next Steps

1. **Explore the interface**: Try registering a service and searching for others
2. **Read the documentation**: Check the README.md for detailed information
3. **Join the community**: Look for community discussions and support
4. **Contribute**: Consider contributing to the project

## Support

If you encounter issues:

1. Check this installation guide
2. Read the README.md file
3. Check the troubleshooting section above
4. Create an issue on the project repository
5. Join community discussions

## System Requirements

- Windows 10 or later
- 4GB RAM minimum (8GB recommended)
- 1GB free disk space
- Internet connection for initial build 