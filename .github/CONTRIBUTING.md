# Contributing to DropSlot

Thank you for your interest in contributing to DropSlot! This document provides guidelines and information for contributors.

## 🚀 Quick Start

1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Create a feature branch** from `main`
4. **Make your changes** with tests
5. **Submit a pull request**

## 📋 Before You Start

- Check existing [issues](https://github.com/ViezeVingertjes/dropslot/issues) and [pull requests](https://github.com/ViezeVingertjes/dropslot/pulls)
- For major changes, open an issue first to discuss your approach
- Read our [Code of Conduct](CODE_OF_CONDUCT.md)

## 🔧 Development Setup

### Prerequisites

- **Rust** (latest stable version)
- **Git**
- **A GitHub account**

### Local Development

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/dropslot.git
cd dropslot

# Create a feature branch
git checkout -b feature/your-feature-name

# Install dependencies and run tests
cargo test

# Run benchmarks
cargo bench

# Check formatting and linting
cargo fmt --check
cargo clippy -- -D warnings
```

## 🎯 Types of Contributions

We welcome several types of contributions:

### 🐛 Bug Reports
- Use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- Include minimal reproduction code
- Specify your environment details

### ✨ Feature Requests
- Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- Provide clear use cases and motivation
- Consider API design implications

### 🚀 Performance Improvements
- Use the [performance template](.github/ISSUE_TEMPLATE/performance.md)
- Include benchmarks showing the improvement
- Consider both latency and throughput impacts

### 📚 Documentation
- Fix typos, improve clarity, add examples
- Update rustdoc comments for public APIs
- Add or improve README sections

### 🔧 Code Contributions
- Bug fixes
- New features
- Performance optimizations
- Code refactoring

## 📝 Code Style

### Rust Guidelines
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Address all `cargo clippy` warnings
- Write comprehensive rustdoc comments for public APIs

### Code Quality
- **Write tests** for new functionality
- **Maintain test coverage** for existing code
- **Add benchmarks** for performance-critical code
- **Document public APIs** with examples

### Commit Messages
Follow [Conventional Commits](https://conventionalcommits.org/):

```
feat: add new subscriber filtering API
fix: resolve race condition in topic cleanup
docs: update performance benchmarks in README
test: add integration tests for concurrent access
perf: optimize topic lookup with better hashing
```

## 🧪 Testing

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests without default features
cargo test --no-default-features

# Run benchmarks
cargo bench
```

### Test Requirements
- **Unit tests** for individual functions/modules
- **Integration tests** for API workflows
- **Benchmarks** for performance-critical code
- **Documentation tests** for code examples

### Test Coverage
- Aim for high test coverage on new code
- Include edge cases and error conditions
- Test both sync and async code paths

## 📊 Benchmarks

### Running Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suites
cargo bench --bench bus
cargo bench --bench topic
cargo bench --bench sub

# Fast benchmarks for CI
CARGO_BENCH_FAST=1 cargo bench
```

### Benchmark Guidelines
- Use `criterion` for statistical rigor
- Include both latency and throughput metrics
- Test realistic scenarios and edge cases
- Document performance characteristics

## 📋 Pull Request Process

### Before Submitting
1. **Sync with upstream**: `git pull upstream main`
2. **Run tests**: `cargo test`
3. **Check formatting**: `cargo fmt --check`
4. **Check linting**: `cargo clippy -- -D warnings`
5. **Run benchmarks**: `cargo bench` (if applicable)

### PR Requirements
- **Clear description** of changes and motivation
- **Reference related issues** (e.g., "Fixes #123")
- **Include tests** for new functionality
- **Update documentation** as needed
- **Follow the PR template**

### Review Process
1. **Automated checks** must pass (CI, formatting, clippy)
2. **Code review** by maintainers
3. **Discussion** and iteration if needed
4. **Approval** and merge

## 🔍 Code Review

### For Contributors
- Be responsive to feedback
- Keep discussions focused and constructive
- Update your PR based on review comments
- Ask questions if requirements are unclear

### For Reviewers
- Be constructive and helpful
- Focus on code quality, performance, and maintainability
- Suggest improvements with examples
- Acknowledge good work

## 📚 Documentation

### API Documentation
- Write clear rustdoc comments for public APIs
- Include usage examples in doc comments
- Document error conditions and edge cases

### README and Guides
- Keep examples up-to-date
- Include performance characteristics
- Add troubleshooting information

## 🎯 Performance Considerations

DropSlot is a high-performance library. Consider:

### Latency
- Minimize allocations in hot paths
- Use appropriate data structures
- Consider CPU cache effects

### Throughput
- Optimize for batch operations
- Consider parallel processing
- Minimize contention in concurrent scenarios

### Memory
- Use `Arc` and `Weak` references appropriately
- Consider memory usage patterns
- Implement proper cleanup

## 🌟 Recognition

Contributors will be recognized in:
- **Git history** with proper attribution
- **Release notes** for significant contributions
- **README** acknowledgments (for major features)

## 📞 Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Code Review**: For implementation guidance

## 🔗 Useful Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/)
- [Criterion Benchmarking](https://bheisler.github.io/criterion.rs/book/)

---

Thank you for contributing to DropSlot! 🏗️
