# Contributing to IP API

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved.

## Code of Conduct

This project and everyone participating in it is governed by respect and professionalism. Please be kind and constructive in your interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates.

When creating a bug report, please include:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected behavior**
- **Actual behavior**
- **System information** (OS, Rust version, etc.)
- **Log output** if applicable

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Clear title and description**
- **Use case** - why is this enhancement needed?
- **Possible implementation** - if you have ideas on how to implement it
- **Alternative solutions** you've considered

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following the coding standards below
3. **Add tests** if applicable
4. **Update documentation** if you're changing functionality
5. **Run tests and linting** before submitting
6. **Write a clear commit message** following our commit message guidelines
7. **Submit your pull request**

## Development Setup

### Prerequisites

- Rust 1.85.0 or later
- Cargo (comes with Rust)

### Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/ip-api.git
cd ip-api

# Build the project
cargo build

# Run tests
cargo test

# Run the application
cargo run -- --port 7111
```

## Coding Standards

### Rust Style Guide

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `cargo fmt` to format your code
- Use `cargo clippy` to catch common mistakes
- Use American English for all code, comments, and documentation

### Code Formatting

Before committing, run:

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Run tests
cargo test
```

### Documentation

- Add doc comments (`///`) for all public functions, structs, and modules
- Include examples in doc comments when helpful
- Keep comments clear and concise
- Update README.md if you're adding new features

### Testing

- Write unit tests for new utility functions
- Add integration tests for new endpoints
- Ensure all tests pass before submitting PR
- Aim for good test coverage of critical paths

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect code meaning (formatting, etc.)
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Changes to build process or auxiliary tools

### Examples

```
feat: add health check endpoint

- Add /health endpoint for monitoring
- Return status and uptime information
- Useful for load balancers and container orchestration
```

```
fix: sanitize IP addresses before lookup

- Validate IP format before reverse DNS lookup
- Prevent invalid IP addresses from causing errors
- Add unit tests for IP validation
```

```
docs: update NGINX configuration guide

- Add rate limiting examples
- Include SSL configuration
- Add troubleshooting section
```

### Breaking Changes

If your change introduces breaking changes, add `BREAKING CHANGE:` in the footer:

```
feat: change API response format

BREAKING CHANGE: Response field names are now camelCase instead of 
PascalCase to follow JSON conventions. Update your clients accordingly.
```

## Project Structure

```
ip-api/
├── src/
│   ├── config.rs           # Configuration management
│   ├── handlers/           # HTTP request handlers
│   │   ├── health.rs
│   │   ├── ip.rs
│   │   ├── metrics.rs
│   │   └── ...
│   ├── middleware/         # HTTP middleware
│   │   ├── logging.rs
│   │   ├── rate_limit.rs
│   │   └── ...
│   ├── models.rs           # Data structures
│   ├── utils/              # Utility functions
│   │   ├── cache.rs
│   │   ├── dns.rs
│   │   ├── security.rs
│   │   └── ...
│   └── main.rs             # Application entry point
├── conf/                   # Server configuration examples
├── build.sh                # Build script
├── Cargo.toml              # Rust dependencies
└── README.md               # Project documentation
```

## Adding New Features

When adding a new feature:

1. **Create a new module** if it's a significant feature
2. **Add unit tests** in the same file using `#[cfg(test)]`
3. **Update documentation** in README.md
4. **Add configuration** to Config struct if needed
5. **Update CHANGELOG.md** with your changes
6. **Add examples** to help users understand the feature

## Module Guidelines

### Handlers

- One file per endpoint group
- Export handler functions as `pub async fn`
- Include comprehensive doc comments
- Return proper HTTP status codes

### Utils

- Keep utility functions focused and single-purpose
- Include unit tests
- Document parameters and return values
- Make functions reusable

### Middleware

- Middleware should be composable
- Document what the middleware does and when to use it
- Include error handling

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests in release mode
cargo test --release
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_example() {
        let result = my_async_function().await;
        assert!(result.is_ok());
    }
}
```

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release date
3. Create a git tag: `git tag v2.0.0`
4. Push tag: `git push origin v2.0.0`
5. GitHub Actions will automatically build and create a release

## Questions?

If you have questions, feel free to:

- Open an issue for discussion
- Reach out to the maintainers

## License

By contributing, you agree that your contributions will be licensed under the EUPL-1.2 License.

## Recognition

All contributors will be recognized in the project. Thank you for making this project better! 🎉