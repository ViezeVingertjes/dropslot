# Contributing to DropSlot

Thank you for your interest in contributing to DropSlot! This document provides guidelines for contributing to the project.

## Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/your-username/dropslot.git
   cd dropslot
   ```

2. **Install Rust**
   Ensure you have Rust installed. We recommend using [rustup](https://rustup.rs/).

3. **Install Dependencies**
   ```bash
   cargo build
   ```

4. **Run Tests**
   ```bash
   cargo test --all-features
   ```

## Code Style

- Follow the official Rust formatting guidelines
- Run `cargo fmt` before submitting
- Run `cargo clippy` to check for lints
- Ensure all tests pass with `cargo test --all-features`

## Pull Request Process

1. **Create a Feature Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Write clear, concise commit messages
   - Include tests for new functionality
   - Update documentation as needed

3. **Test Your Changes**
   ```bash
   cargo test --all-features
   cargo clippy --all-features -- -D warnings
   cargo fmt --all -- --check
   ```

4. **Submit Pull Request**
   - Include a clear description of changes
   - Reference any related issues
   - Ensure CI passes

## Code Guidelines

### Performance
- Use `#[inline]` on small, frequently called functions
- Prefer zero-copy operations where possible
- Consider memory layout and cache efficiency

### Error Handling
- Use `Result<T, BusError>` for fallible operations
- Provide meaningful error messages
- Document error conditions in function docs

### Testing
- Write unit tests for all public functions
- Include integration tests for complex scenarios
- Add doc tests for example code
- Aim for high test coverage

### Documentation
- Include rustdoc comments for all public items
- Provide usage examples in doc comments
- Keep examples simple and focused
- Update README.md for significant changes

## Reporting Issues

When reporting bugs, please include:
- Rust version (`rustc --version`)
- Operating system
- Minimal reproducible example
- Expected vs actual behavior

## Feature Requests

For new features:
- Open an issue first to discuss the proposal
- Consider performance implications
- Ensure it aligns with project goals
- Include use cases and examples

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

## Getting Help

- Check existing issues and documentation
- Ask questions in issues with the "question" label
- Join the discussion in pull requests

Thank you for contributing to DropSlot! 
