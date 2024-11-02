# Quick Start Guide

## 5-Minute Setup

1. **Install Rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. **Get XCR**:
```bash
git clone https://github.com/serkanaltuntas/xcr.git
cd xcr
cargo build --release
```

3. **Run XCR**:
```bash
./target/release/xcr /path/to/OldProject NewProject --bundle-id com.company.newproject
```

## Example Session

Here's a complete example of renaming a project:

```bash
# Build XCR
cargo build --release

# Rename a project
./target/release/xcr /Users/<user-folder>/Developer/MyApp NewApp --bundle-id com.company.newapp

# Check the log file (will be in current directory)
cat xcr_20240102_123456.log

# Open new project in Xcode
open /Users/<user-folder>/Developer/NewApp/NewApp.xcodeproj
```

That's it! Your project is renamed and ready to use.

## Need Help?

- Check the full README.md for detailed documentation
- Review the log file for operation details
- Ensure Xcode is closed before running
- Make sure you have proper permissions
