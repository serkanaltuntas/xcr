# 🔨 Xcode Carpenter (xcr)

[![Crates.io](https://img.shields.io/crates/v/xcr.svg)](https://crates.io/crates/xcr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Xcode Carpenter is your trusty companion for Xcode project maintenance. Like a skilled carpenter, it carefully handles your Xcode projects with precision and care. Currently, it specializes in project renaming while preserving all references and configurations.

## 🛠️ Features

- 🏗️ Precise project renaming with reference preservation
- 🔄 Automatic bundle identifier updates
- 📝 Detailed operation logging
- ⚡ Blazingly fast (written in Rust)
- 🛡️ Safe operations with automatic rollback
- 🧰 Extensible toolset (more features coming soon)

## 🔧 Installation

### Using Cargo

```bash
cargo install xcr
```

### From Source

```bash
git clone https://github.com/serkanaltuntas/xcr.git
cd xcr
cargo build --release
```

The binary will be available at `target/release/xcr`

## 🚀 Usage

Basic project renaming:
```bash
xcr /path/to/your/project NewProjectName
```

With bundle identifier update:
```bash
xcr /path/to/your/project NewProjectName --bundle-id com.company.newname
```

For more detailed instructions, check out the [Quick Start Guide](QUICK_START.md).

## 📋 Requirements

- Rust 1.70 or higher
- Xcode Command Line Tools
- Administrative privileges (for some operations)

## 📚 Documentation

- [Quick Start Guide](QUICK_START.md)
- [Contributing Guidelines](CONTRIBUTING.md)
- [Release Notes](CHANGELOG.md)

## ⚙️ How It Works

Xcode Carpenter follows these careful steps:

1. 📋 Validates the source project structure
2. 📦 Creates a precise copy of the project
3. 🔄 Updates all file references
4. 📝 Renames files and directories
5. 🏷️ Updates bundle identifiers (when specified)
6. 🧹 Performs cleanup if needed

All operations are meticulously logged for transparency.

## 🛡️ Safety Features

- 💾 Creates backups before modifications
- ↩️ Automatic rollback on failure
- 📝 Comprehensive logging
- ✅ Project structure validation

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) before submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

- Serkan Altuntas ([@serkanaltuntas](https://github.com/serkanaltuntas))

## 🙏 Acknowledgments

- Built with Rust 🦀
- Inspired by real-world Xcode project maintenance needs

## 🗺️ Roadmap

Future tools planned for the Xcode Carpenter's toolbox:
- 🔧 Project dependency analyzer
- 🎨 Asset catalog optimizer
- 📦 Framework integrator
- 🧪 Test target generator
- 🔍 Code duplicates finder

Stay tuned for more features!
