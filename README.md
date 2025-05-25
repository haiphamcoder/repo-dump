# Repo Dump

```
██████╗ ███████╗██████╗  ██████╗     ██████╗ ██╗   ██╗███╗   ███╗██████╗ 
██╔══██╗██╔════╝██╔══██╗██╔═══██╗    ██╔══██╗██║   ██║████╗ ████║██╔══██╗
██████╔╝█████╗  ██████╔╝██║   ██║    ██║  ██║██║   ██║██╔████╔██║██████╔╝
██╔══██╗██╔══╝  ██╔═══╝ ██║   ██║    ██║  ██║██║   ██║██║╚██╔╝██║██╔═══╝ 
██║  ██║███████╗██║     ╚██████╔╝    ██████╔╝╚██████╔╝██║ ╚═╝ ██║██║     
╚═╝  ╚═╝╚══════╝╚═╝      ╚═════╝     ╚═════╝  ╚═════╝ ╚═╝     ╚═╝╚═╝     
```

A powerful Rust-based tool for analyzing and dumping source code from project directories. This tool automatically detects project technologies, generates directory trees, and creates comprehensive source code dumps.

## ✨ Features

- 🔍 **Automatic Technology Detection**
  - Supports multiple programming languages and frameworks
  - Smart detection of project structure and dependencies
  - Extensible technology indicators

- 📁 **Directory Tree Generation**
  - Beautiful ASCII tree visualization
  - Intelligent exclusion of unnecessary directories
  - Customizable tree depth and format

- 🌐 **Multilingual Support**
  - English and Vietnamese interfaces
  - Easy to add more languages
  - Consistent user experience across languages

- 🛠️ **Smart File Handling**
  - Automatic file size limits
  - Intelligent file type filtering
  - Proper handling of binary and large files

- 📊 **Comprehensive Output**
  - Detailed project structure
  - Complete source code dump
  - Statistics and summaries

## 🚀 Installation

### Prerequisites
- Rust 1.85.0 or higher
- Cargo (Rust's package manager)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/haiphamcoder/repo-dump.git
cd repo-dump

# Build the project
cargo build --release

# The binary will be available at target/release/repo-dump
```

## 💻 Usage

### Basic Usage
```bash
# Run with default settings (interactive mode)
cargo run

# Specify project path directly
cargo run -- --path /path/to/project

# Using the built binary
./target/release/repo-dump-rs --path /path/to/project
```

### Command Line Options
- `--path`, `-p`: Specify the project path to analyze
- `--help`: Show help message
- `--version`: Show version information

### Interactive Mode
1. Select language (en/vi)
2. Enter project path (or press Enter to use current directory)
3. Wait for analysis to complete
4. Find the output in `source_dump.txt`

## 🤝 Contributing

### Branch Structure
- `main`: Stable branch, contains production-ready code
- `feature-dev`: Integration branch for features
- Feature branches: `feature/*` for specific features

### Development Workflow
1. Create a new branch from `feature-dev`:
   ```bash
   git checkout feature-dev
   git pull
   git checkout -b feature/your-feature-name
   ```

2. Make your changes and commit:
   ```bash
   git add .
   git commit -m "feat: add new feature"
   ```

3. Push and create a Pull Request:
   ```bash
   git push origin feature/your-feature-name
   ```

4. After review and approval, your PR will be merged into `feature-dev`
5. When ready for release, `feature-dev` will be merged into `main`

### Commit Message Guidelines
We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

Example:
```
feat(tech-detection): add support for Rust projects

- Add Rust-specific file patterns
- Implement Rust project detection
- Add tests for Rust detection

Closes #123
```

### Pull Request Guidelines
1. Use the PR template
2. Link related issues
3. Keep PRs focused and small
4. Update documentation
5. Add tests if needed
6. Follow the commit message guidelines

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by various code analysis tools
- Built with Rust's excellent ecosystem
- Thanks to all contributors

## 📞 Support

For support, please:
- Open an issue on GitHub
- Check the documentation
- Contact the maintainers
