# Changelog

All notable changes to Xcode Carpenter will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-11-02

### Added
- Initial release of Xcode Carpenter
- CLI tool for Xcode project renaming with comprehensive features:
    - Project path and new name processing
    - Optional bundle identifier updating
    - Comprehensive file system operations
    - Detailed logging system
- Command-line interface using `clap` with support for:
    - Project path argument
    - New name argument
    - Optional bundle ID flag
- Logging system with `log4rs` featuring:
    - File and console output
    - Timestamped log files
    - Detailed operation logging
- Error handling with `anyhow` for user-friendly error messages
- Project validation and safety checks
- Documentation:
    - MIT License file
    - Comprehensive README
    - Quick Start guide
    - Basic project setup

### Changed
- Updated project description to "Xcode Carpenter: A tool for renaming Xcode projects"
- Updated repository URLs in documentation

### Technical Details
#### Dependencies
- Added core dependencies:
    - `anyhow` for error handling
    - `clap` for CLI parsing
    - `log` and `log4rs` for logging
    - `glob` for file operations
    - `chrono` for timestamp formatting

#### Features
- Project configuration storage with `XcrConfig`
- Automated file operations:
    - Project validation
    - Bundle identifier detection
    - File content updates
    - Directory structure preservation
- Comprehensive error handling and recovery
- Logging configuration with both file and console outputs

[0.1.0]: https://github.com/serkanaltuntas/xcr/releases/tag/v0.1.0
