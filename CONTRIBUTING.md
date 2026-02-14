# Contributing to Ethereum Boilerplate — Rust Edition

We welcome contributions to the Ethereum Boilerplate Rust project! This guide will help you get started.

## 🤝 How to Contribute

### Reporting Issues

- Use the [GitHub Issues](https://github.com/revitalyr/nodejs_2_rust/issues) page
- Provide clear, descriptive titles
- Include steps to reproduce
- Add relevant error messages and logs
- Specify your environment (OS, Rust version, etc.)

### Submitting Pull Requests

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Make** your changes
4. **Test** thoroughly:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```
5. **Commit** your changes:
   ```bash
   git commit -m "feat: add amazing feature"
   ```
6. **Push** to your fork:
   ```bash
   git push origin feature/amazing-feature
   ```
7. **Create** a Pull Request

## 🛠 Development Setup

### Prerequisites

- Rust 1.70+
- Node.js 18+ (for frontend tooling)
- Git

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/nodejs_2_rust.git
cd nodejs_2_rust

# Add upstream remote
git remote add upstream https://github.com/revitalyr/nodejs_2_rust.git

# Install dependencies
cargo build

# Run tests
cargo test
```

### Project Structure

```
crates/
├── cli/              # CLI tools and commands
├── server/            # Axum REST API
├── frontend/          # Leptos WASM frontend
├── shared/            # Shared types and utilities
├── utils/             # Common helper functions
└── smart-contracts/   # Ethereum contract utilities
```

## 📝 Coding Standards

### Rust Guidelines

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write comprehensive tests
- Document public APIs with `///` doc comments

### Code Style

```rust
// Good
pub fn calculate_balance(amount: U256) -> Result<U256, Error> {
    // Clear, documented function
    amount.checked_mul(U256::from(2))
        .ok_or_else(|| Error::Overflow)
}

// Avoid
pub fn calc(a: U256) -> U256 {
    a * 2  // No error handling, unclear name
}
```

### Documentation

- All public functions must have documentation
- Include examples in doc comments
- Explain the "why" not just the "what"
- Use proper markdown formatting

## 🧪 Testing

### Test Categories

- **Unit tests**: Test individual functions
- **Integration tests**: Test component interaction
- **WASM tests**: Test frontend in browser

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = U256::from(100);
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, U256::from(200));
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p ethereum-boilerplate-utils

# Integration tests
cargo test --test integration

# WASM tests
wasm-pack test --headless --firefox
```

## 🚀 Release Process

We use [Semantic Versioning](https://semver.org/):

- **Major**: Breaking changes
- **Minor**: New features (backward compatible)
- **Patch**: Bug fixes

### Release Checklist

- [ ] All tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] Version bumped in Cargo.toml
- [ ] Git tag created

## 📋 Types of Contributions

### 🐛 Bug Fixes

- Fix crashes or incorrect behavior
- Improve error messages
- Fix memory leaks or performance issues

### ✨ Features

- New CLI commands
- Additional contract templates
- New UI components
- Enhanced API endpoints

### 📚 Documentation

- Improve README
- Add code examples
- Write tutorials
- Fix typos or unclear explanations

### 🛠 Infrastructure

- Improve CI/CD
- Add new test types
- Optimize build process
- Enhance developer tooling

## 🏷 Labels

We use these labels for issues and PRs:

- `bug`: Bug reports
- `enhancement`: Feature requests
- `documentation`: Documentation improvements
- `good first issue`: Good for newcomers
- `help wanted`: Community assistance needed

## 📧 Getting Help

- **Discord**: [Join our community](https://discord.gg/rust-web3)
- **Discussions**: [GitHub Discussions](https://github.com/revitalyr/nodejs_2_rust/discussions)
- **Issues**: [Create an issue](https://github.com/revitalyr/nodejs_2_rust/issues)

## 🙏 Recognition

Contributors are recognized in:

- README.md contributors section
- Release notes
- Annual community posts
- Special contributor badges

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Ethereum Boilerplate — Rust Edition! 🚀
