# Xmake Language Server

A language server designed specifically for the XMake build system, developed based on the EmmyLua project, providing powerful IDE support for XMake Lua scripts.

## 🚀 Features

### Core Features
- **Intelligent Code Completion** - Smart completion for XMake APIs, including functions, configuration options, and parameter hints
- **Syntax Highlighting** - Syntax highlighting for XMake Lua scripts
- **Error Detection** - Real-time detection of syntax errors and XMake configuration issues
- **Code Navigation** - Supports go to definition, find references, and symbol navigation
- **Hover Information** - Displays function documentation, parameter descriptions, and usage examples

### XMake-Specific Features
- **XMake API Support** - Complete XMake 2.7.7+ API definitions and documentation
- **Target Configuration Suggestions** - Smart completion for target, package, option, and other configurations
- **Platform & Architecture Awareness** - Smart suggestions for cross-platform configurations
- **Dependency Package Management** - TODO: Smart support for package.lua and dependency configuration
- **Build Rule Support** - TODO: Code completion for custom build rules

## 📦 Installation

### Build from Source

Make sure you have the Rust toolchain installed (recommended 1.89+):

```bash
# Clone the repository
git clone https://github.com/CppCXY/xmake_ls.git
cd xmake_ls

# Build the project
cargo build --release -p xmake_ls

# The executable will be generated at target/release/xmake_ls
```

## ⚡ Quick Start

todo

## 🔧 Development

### Project Structure

```
xmake_ls/
├── crates/
│   ├── xmake_ls/             # Main language server
│   └── xmake_code_analysis/  # Code analysis core
```

### Build and Test

```bash
# Run tests
cargo test

# Check code formatting
cargo fmt --check
```

### Contribution Guide

1. Fork the project and create a feature branch
2. Write code and add tests
3. Ensure all tests pass
4. Submit a Pull Request

## 📄 License

This project is licensed under MIT. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributors

Thanks to all developers who contributed to this project!

## 📞 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/CppCXY/xmake_ls/issues)
- **Discussions**: [GitHub Discussions](https://github.com/CppCXY/xmake_ls/discussions)
- **XMake Official**: [XMake Website](https://xmake.io)

## 🔗 Related Projects

- [XMake](https://github.com/xmake-io/xmake) - Lightweight cross-platform build tool based on Lua
- [EmmyLua](https://github.com/EmmyLuaLs/emmylua-analyzer-rust) - The original EmmyLua language server project

---

If you find this project helpful, please consider giving us a ⭐ Star!