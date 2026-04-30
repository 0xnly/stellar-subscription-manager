# Contributing to StellarSubs

Thank you for your interest in contributing to StellarSubs! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/0xnly/stellar-subscription-manager/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)
   - Relevant logs or screenshots

### Suggesting Features

1. Check existing [Issues](https://github.com/0xnly/stellar-subscription-manager/issues) for similar suggestions
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Potential implementation approach

### Pull Requests

1. **Fork the repository**
   ```bash
   git clone https://github.com/0xnly/stellar-subscription-manager.git
   cd stellar-subscription-manager
   ```

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**
   - Write clear, documented code
   - Follow Rust style guidelines
   - Add tests for new functionality
   - Update documentation as needed

4. **Test your changes**
   ```bash
   cargo test
   cargo fmt
   cargo clippy
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

   Use conventional commit messages:
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation changes
   - `test:` Test additions or changes
   - `refactor:` Code refactoring
   - `chore:` Maintenance tasks

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your branch
   - Fill in the PR template
   - Link related issues

## Development Setup

### Prerequisites

- Rust 1.94+
- Stellar CLI
- Git

### Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/stellar-subscription-manager.git
cd stellar-subscription-manager

# Build
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Pass all clippy lints (`cargo clippy`)
- Write clear, self-documenting code
- Add comments for complex logic
- Document all public functions

## Testing

- Write unit tests for all new functions
- Maintain 100% test coverage
- Test edge cases and error conditions
- Run full test suite before submitting PR

```bash
cargo test
```

## Documentation

- Update README.md for user-facing changes
- Update inline documentation for code changes
- Add examples for new features
- Keep documentation clear and concise

## Review Process

1. Maintainers will review your PR
2. Address any requested changes
3. Once approved, your PR will be merged
4. Your contribution will be credited

## Questions?

- Open an issue for questions
- Join [Stellar Discord](https://discord.gg/stellar)
- Check [Soroban Documentation](https://soroban.stellar.org/)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to StellarSubs! 🚀
