# Contributing to XCR

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
    - [Development Setup](#development-setup)
    - [Project Structure](#project-structure)
- [Making Contributions](#making-contributions)
    - [Finding Issues](#finding-issues)
    - [Pull Requests](#pull-requests)
- [Development Guidelines](#development-guidelines)
    - [Coding Style](#coding-style)
    - [Commit Messages](#commit-messages)
    - [Documentation](#documentation)
    - [Testing](#testing)
- [Release Process](#release-process)

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to serkan@serkan.ai.

## Getting Started

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/serkanaltuntas/xcr.git
   cd xcr
   ```

2. **Install development dependencies**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install development tools
   cargo install cargo-watch    # For development with auto-recompile
   cargo install cargo-audit    # For security audits
   cargo install cargo-outdated # For dependency checking
   cargo install cargo-tarpaulin # For code coverage
   ```

3. **Build the project**
   ```bash
   # Debug build
   cargo build
   
   # Release build
   cargo build --release
   ```

### Project Structure

```
xcr/
├── src/
│   └── main.rs               # Main application code
├── CHANGELOG.md             # Project changelog
├── CONTRIBUTING.md         # Contribution guidelines
├── Cargo.toml             # Project metadata and dependencies
├── LICENSE               # MIT License
├── QUICK_START.md       # Quick start guide
└── README.md           # Project documentation
```

## Making Contributions

### Finding Issues

- Check existing issues in the [XCR Issue Tracker](https://github.com/serkanaltuntas/xcr/issues)
- Look for issues labeled `good first issue` for beginners
- Feel free to ask questions in issues

### Pull Requests

1. **Create a branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
    - Write clear, concise commit messages following our convention
    - Include tests for new functionality
    - Update documentation as needed

3. **Run checks locally**
   ```bash
   # Format code
   cargo fmt
   
   # Run clippy lints
   cargo clippy -- -D warnings
   
   # Run tests
   cargo test
   
   # Check security vulnerabilities
   cargo audit
   ```

4. **Push and create PR**
   ```bash
   git push origin feature/your-feature-name
   ```
   Then create a Pull Request through GitHub.

## Development Guidelines

### Coding Style

- Follow the Rust standard style guide
- Use `cargo fmt` before committing
- Run `cargo clippy` and address all warnings
- Keep functions focused and small
- Use meaningful variable names
- Add comments for complex logic
- Follow the existing code structure in main.rs

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes
- `refactor`: Code changes that neither fix bugs nor add features
- `perf`: Performance improvements
- `test`: Adding or modifying tests
- `chore`: Maintenance tasks

Example:
```
feat(rename): add support for workspace renaming

Added capability to rename Xcode workspace files along with projects.
Includes updates to all workspace references and schemes.

Closes #123
```

### Documentation

- Document all public functions and types with rustdoc comments
- Keep README.md up to date with new features
- Update CHANGELOG.md for each release
- Add examples for complex functionality
- Update QUICK_START.md for user-facing changes

### Testing

1. **Run all tests**
   ```bash
   cargo test
   ```

2. **Run specific test**
   ```bash
   cargo test test_name
   ```

3. **Generate test coverage**
   ```bash
   cargo tarpaulin
   ```

## Release Process

1. **Prepare the release**
    - Update version in Cargo.toml
    - Update CHANGELOG.md
    - Ensure all tests pass

2. **Create release**
   ```bash
   # Tag the release
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   
   # Publish to crates.io
   cargo login
   cargo publish
   ```

## Questions?

Feel free to:
- Open an issue on [GitHub](https://github.com/serkanaltuntas/xcr/issues)
- Email the maintainer at serkan@serkan.ai

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
