# GitHub Workflows Documentation

This directory contains comprehensive CI/CD workflows for the egui OpenGL application.

## Workflow Overview

### 🔄 `ci.yml` - Continuous Integration
**Triggers:** Push to main/develop, Pull Requests
**Purpose:** Core testing and validation

- **Cross-platform testing** (Ubuntu, Windows, macOS)
- **Multiple Rust versions** (stable, beta)
- **Code quality checks** (clippy, rustfmt)
- **Build validation**
- **CLI argument testing**
- **Documentation generation**
- **Performance monitoring**

### 🚀 `release.yml` - Release Automation
**Triggers:** Git tags (v*.*.*), Manual dispatch
**Purpose:** Automated release creation

- **Multi-platform binary builds**
- **GitHub release creation**
- **Asset packaging** with documentation
- **Checksum generation**
- **Post-release testing**

### 🔒 `security.yml` - Security Scanning
**Triggers:** Weekly schedule, Push to main, PRs
**Purpose:** Security and compliance

- **Dependency vulnerability scanning** (cargo-audit)
- **License compliance checking**
- **Outdated dependency detection**
- **CodeQL static analysis**
- **SARIF reporting**

### 🌙 `nightly.yml` - Advanced Testing
**Triggers:** Daily at 2 AM UTC, Manual dispatch
**Purpose:** Comprehensive testing and analysis

- **Rust nightly compatibility**
- **Minimal dependency versions testing**
- **Memory leak detection** (Valgrind)
- **Fuzz testing**
- **Performance benchmarking**
- **Automated reporting**

## Key Features

### 🎯 GUI Application Challenges Solved

1. **Headless Testing:** Uses `xvfb` on Linux for GUI apps without display
2. **OpenGL Dependencies:** Proper system library installation
3. **Cross-platform Builds:** Handles Windows/Linux/macOS differences
4. **Performance Monitoring:** Binary size and startup time tracking
5. **Security Focus:** Regular vulnerability scanning

### 📊 Testing Strategy

```
Unit Tests → Integration Tests → Performance Tests → Security Tests
     ↓              ↓                ↓                 ↓
   Quick          Realistic      Regression        Compliance
  Feedback        Scenarios       Detection         Monitoring
```

### 🏗️ Build Matrix

| OS | Rust Version | Purpose |
|---|---|---|
| Ubuntu | stable, beta | Primary development platform |
| Windows | stable | Windows compatibility |
| macOS | stable | macOS compatibility |
| Ubuntu | nightly | Future compatibility testing |

## Usage Instructions

### Running Tests Locally

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# Security audit
cargo install cargo-audit
cargo audit

# Format check
cargo fmt --check

# Linting
cargo clippy -- -D warnings
```

### Manual Release

1. Create and push a git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. Or trigger manually via GitHub Actions UI

### Understanding CI Results

- ✅ **Green:** All tests pass, ready to merge
- ⚠️ **Yellow:** Some non-critical issues (warnings, nightly failures)
- ❌ **Red:** Critical failures, do not merge

## Monitoring and Alerts

### Performance Regression Detection
- Binary size tracking
- Startup time monitoring
- Compilation time benchmarks
- Automatic PR comments with metrics

### Security Monitoring
- Weekly vulnerability scans
- License compliance tracking
- Dependency freshness monitoring
- CodeQL security analysis

### Quality Gates

**Before Merge:**
- [ ] All platform builds pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] Integration tests pass
- [ ] No security vulnerabilities

**Release Criteria:**
- [ ] All CI checks pass
- [ ] Security audit clean
- [ ] Performance within acceptable bounds
- [ ] Documentation updated

## Customization

### Adding New Platforms
Edit the build matrix in `ci.yml` and `release.yml`:

```yaml
strategy:
  matrix:
    include:
      - os: your-new-platform
        target: your-rust-target
```

### Adding New Tests
1. Add unit tests in `src/` files with `#[cfg(test)]`
2. Add integration tests in `tests/` directory (e.g., `integration_tests.rs`)

### Modifying Security Policies
Update `security.yml` to adjust:
- Scan frequency
- Vulnerability thresholds
- License allowlists
- Audit scope

## Troubleshooting

### Common Issues

1. **OpenGL Dependencies Missing**
   - Solution: Update Linux dependency installation in workflows

2. **Windows Build Failures**
   - Often related to path separators or PowerShell syntax
   - Use `shell: powershell` for Windows-specific commands

3. **GUI Tests Failing**
   - Ensure `xvfb` is used on Linux for headless testing
   - Consider using `timeout` for GUI apps that might hang

4. **Memory Test Failures**
   - Valgrind can be sensitive to system libraries
   - May need to add suppressions for known false positives

### Debug Tips

1. **Local Reproduction:** Try to reproduce CI failures locally first
2. **Artifact Downloads:** Use workflow artifacts to debug build issues
3. **Matrix Debugging:** Disable some matrix combinations to isolate issues
4. **Verbose Logging:** Add `RUST_BACKTRACE=full` for detailed error info

## Best Practices

1. **Fast Feedback:** Keep CI builds under 10 minutes when possible
2. **Fail Fast:** Put quick checks (format, clippy) before slow ones (tests)
3. **Cache Smartly:** Use appropriate cache keys for Rust builds
4. **Resource Limits:** Set reasonable timeouts to prevent hanging
5. **Clear Documentation:** Document any special requirements or setup

This workflow setup provides production-ready CI/CD for a Rust GUI application with comprehensive testing, security, and release automation.