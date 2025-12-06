# Development Guide

This document provides complete command reference and best practices for NeuraDock development.

## Table of Contents

- [Quick Start](#quick-start)
- [Complete Command Reference](#complete-command-reference)
- [Common Workflows](#common-workflows)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### First Time Setup

```bash
# 1. Clone the repository
git clone https://github.com/i-rtfsc/NeuraDock.git
cd NeuraDock

# 2. Install dependencies
make setup

# 3. Start development server
make dev
```

### Daily Development

```bash
# Start development server
make dev

# Quick start (skip dependency check)
make dev-fast

# View logs
make logs

# Restart server
make kill dev
```

---

## Complete Command Reference

### 📦 Installation and Dependencies

| Command | Description | Use Case |
|---------|-------------|----------|
| `make setup` | Install all dependencies (first time) | After cloning repository |
| `make install` | Same as setup | Same as setup |
| `make check-deps` | Check if dependencies are installed | Verify environment |
| `make update-deps` | Update all dependencies | Regular maintenance |
| `make outdated` | Check outdated dependencies | See updatable packages |
| `make install-rust-tools` | Install Rust development tools | Setup dev environment |

**Examples:**
```bash
# First time installation
make setup

# Regular dependency updates
make update-deps

# Check which packages are outdated
make outdated
```

### 🚀 Development Mode

| Command | Description | Log Level |
|---------|-------------|-----------|
| `make dev` | Start development mode | info (standard) |
| `make dev-debug` | Start development mode | debug (verbose) |
| `make dev-trace` | Start development mode | trace (tracing) |
| `make dev-warn` | Start development mode | warn (warnings only) |
| `make dev-fast` | Quick start | info (skip checks) |
| `make dev-first` | First run | info (auto install) |
| `make kill` | Kill all processes | - |

**Examples:**
```bash
# Standard development
make dev

# When detailed logs are needed
make dev-debug

# For performance analysis
make dev-trace

# Quick start (for frequent restarts)
make dev-fast

# Force restart
make kill dev
```

### 📦 Build Commands

| Command | Description | Output |
|---------|-------------|--------|
| `make build` | Build Release version | Binary file |
| `make build-release` | Build and package | Installers (.dmg/.msi/.AppImage) |
| `make build-release-fast` | Quick build | Binary (no packaging) |
| `make build-frontend` | Build frontend only | dist/ directory |
| `make build-backend` | Build backend only | target/release/ |
| `make run-release` | Run Release version | - |
| `make rebuild` | Clean and rebuild | Binary file |
| `make bindings` | Generate TypeScript bindings | src/lib/tauri.ts |

**Examples:**
```bash
# Development build (fast)
make build

# Production build (full packaging)
make build-release

# Test Release version
make build-release-fast
make run-release

# Update frontend only
make build-frontend
```

**Build Output Locations:**
- macOS: `apps/desktop/src-tauri/target/release/bundle/dmg/`
- Windows: `apps/desktop/src-tauri/target/release/bundle/msi/`
- Linux: `apps/desktop/src-tauri/target/release/bundle/appimage/`

### 🧪 Test Commands

| Command | Description | Output |
|---------|-------------|--------|
| `make test` | Run all tests | Test results |
| `make test-backend` | Run backend tests | Test results |
| `make test-coverage` | Generate coverage report | HTML/JSON/LCOV |
| `make coverage-report` | Open coverage report | Open in browser |

**Examples:**
```bash
# Quick test
make test-backend

# Generate and view coverage
make test-coverage
make coverage-report
```

**Coverage Report Locations:**
- HTML: `apps/desktop/src-tauri/coverage/tarpaulin-report.html`
- JSON: `apps/desktop/src-tauri/coverage/tarpaulin-report.json`
- LCOV: `apps/desktop/src-tauri/coverage/lcov.info`

### 🧹 Clean Commands

| Command | Description | Removes |
|---------|-------------|---------|
| `make clean` | Clean build artifacts | dist/ + target/ |
| `make clean-frontend` | Clean frontend | dist/ + .vite/ |
| `make clean-backend` | Clean backend | target/ + coverage/ |
| `make clean-all` | Deep clean | Above + node_modules/ + logs + database |

**Examples:**
```bash
# Daily cleanup
make clean

# Complete reset (reinstall dependencies)
make clean-all
make setup
```

**Clean Details:**
- `clean`: Remove build artifacts (~13GB)
- `clean-all`: Remove everything including:
  - `node_modules/` (~350MB)
  - `target/` (~13GB)
  - Log files
  - Database files

### ✅ Code Quality

| Command | Description | Tools |
|---------|-------------|-------|
| `make check` | Check code format | rustfmt + clippy |
| `make fix` | Auto-fix format | rustfmt |

**Examples:**
```bash
# Pre-commit check
make check

# Auto-fix formatting issues
make fix

# Complete check workflow
make fix
make check
make test-backend
```

### 🔧 Tools and Information

| Command | Description | Purpose |
|---------|-------------|---------|
| `make env-check` | Check development environment | Verify tool installation |
| `make version` | Show version information | View version numbers |
| `make status` | View project status | Check dependencies and ports |
| `make migrate` | Run database migrations | Update database schema |
| `make logs` | View today's logs | Debug issues |
| `make fix-permissions` | Fix file permissions | Resolve permission issues |
| `make help` | Show help information | View all commands |

**Examples:**
```bash
# Verify development environment
make env-check

# View version
make version

# View project status
make status

# View logs
make logs

# Get help
make help
```

---

## Common Workflows

### 1. Daily Development Workflow

```bash
# 1. Start development
make dev

# 2. Write code...

# 3. Test
make test-backend

# 4. Check format
make check

# 5. Auto-fix
make fix

# 6. Commit code
git add .
git commit -m "feat: ..."
```

### 2. Release Workflow

```bash
# 1. Update version number
# Edit apps/desktop/src-tauri/Cargo.toml

# 2. Clean old builds
make clean-all

# 3. Reinstall dependencies
make setup

# 4. Run tests
make test-backend

# 5. Build Release version
make build-release

# 6. Test Release version
make run-release

# 7. View build artifacts
ls -lh apps/desktop/src-tauri/target/release/bundle/*/
```

### 3. Debug Workflow

```bash
# 1. Start with debug logs
make dev-debug

# 2. View real-time logs
make logs

# 3. For performance analysis
make kill
make dev-trace

# 4. Run tests to locate issues
make test-backend

# 5. Generate coverage report
make test-coverage
make coverage-report
```

### 4. Dependency Update Workflow

```bash
# 1. Check outdated dependencies
make outdated

# 2. Update dependencies
make update-deps

# 3. Test
make test-backend

# 4. If issues occur, rollback
git checkout -- apps/desktop/package.json
git checkout -- apps/desktop/src-tauri/Cargo.toml
make setup
```

### 5. Complete Reset Workflow

```bash
# 1. Deep clean
make clean-all

# 2. Reinstall dependencies
make setup

# 3. Verify environment
make env-check

# 4. Start development
make dev
```

---

## Troubleshooting

### Issue: Dependency Installation Failed

```bash
# Solution 1: Clean and reinstall
make clean-all
make setup

# Solution 2: Check network and Node version
make env-check
node --version  # Requires >= 20.0.0

# Solution 3: Manual installation
cd apps/desktop
rm -rf node_modules
npm install --legacy-peer-deps
```

### Issue: Development Server Won't Start

```bash
# Solution 1: Kill old processes
make kill
make dev

# Solution 2: Check port usage
lsof -ti:1420  # Tauri port
lsof -ti:5173  # Vite port

# Solution 3: Restart with verbose logs
make dev-debug
```

### Issue: Build Failed

```bash
# Solution 1: Clean and rebuild
make clean
make build

# Solution 2: Complete reset
make clean-all
make setup
make build

# Solution 3: Check Rust version
rustc --version  # Requires >= 1.70.0
cargo --version
```

### Issue: Tests Failed

```bash
# Solution 1: Run single test
cd apps/desktop/src-tauri
cargo test <test_name> -- --nocapture

# Solution 2: Clean test cache
make clean-backend
make test-backend

# Solution 3: View detailed output
cd apps/desktop/src-tauri
RUST_LOG=debug cargo test -- --nocapture
```

### Issue: Permission Error

```bash
# Solution: Fix permissions
make fix-permissions

# Or manually fix
chmod +x apps/desktop/src-tauri/target/release/neuradock
chmod -R u+w apps/desktop/node_modules
```

### Issue: Database Error

```bash
# Solution 1: Re-run migrations
make migrate

# Solution 2: Delete and recreate database
rm *.db *.db-shm *.db-wal
make dev  # Will auto-create database

# Solution 3: Use development database
# Development environment uses neuradock_dev.db
```

---

## Environment Variables

### Log Levels

```bash
# Control log level via RUST_LOG
RUST_LOG=debug make dev      # Verbose logs
RUST_LOG=trace make dev      # Trace level (most detailed)
RUST_LOG=warn make dev       # Warnings only
RUST_LOG=info make dev       # Standard logs (default)

# Or use preset commands
make dev-debug               # Equivalent to RUST_LOG=debug
make dev-trace               # Equivalent to RUST_LOG=trace
make dev-warn                # Equivalent to RUST_LOG=warn
```

### Database Locations

- **Development**: `neuradock_dev.db`
- **Production**:
  - macOS: `~/Library/Application Support/com.neuradock.app/neuradock.db`
  - Windows: `%APPDATA%\com.neuradock.app\neuradock.db`
  - Linux: `~/.local/share/com.neuradock.app/neuradock.db`

### Log Locations

- macOS: `~/Library/Logs/neuradock/logs/`
- Windows: `%APPDATA%\neuradock\logs\`
- Linux: `~/.local/share/neuradock/logs/`

---

## Performance Optimization Tips

### Speed Up Development Startup

```bash
# 1. Use dev-fast to skip dependency check
make dev-fast

# 2. Use Rust incremental compilation (enabled by default)
# Already configured in Cargo.toml

# 3. Use fewer logs
make dev-warn  # Show warnings only
```

### Speed Up Build

```bash
# 1. Build only what's needed
make build-frontend  # Frontend only
make build-backend   # Backend only

# 2. Use Release fast build
make build-release-fast  # Compile but don't package

# 3. Use multi-core compilation
# Rust uses all CPU cores by default
```

### Reduce Disk Usage

```bash
# 1. Regular cleanup
make clean

# 2. Deep clean (when resetting environment)
make clean-all

# 3. Clean Rust cache
cargo cache --autoclean  # Requires cargo-cache
```

---

## Related Documentation

- [Contributing Guide](./contributing.md) - Complete contribution workflow
- [Architecture Overview](./architecture/architecture_overview.md) - System architecture
- [Technical Details](./architecture/technical_details.md) - Technical deep dive
- [API Reference](./api/api_reference.md) - API documentation

---

## Getting Help

```bash
# View all commands
make help

# Check development environment status
make env-check
make status

# View version information
make version
```

For issues, refer to:
- [Troubleshooting Documentation](./user_guide/troubleshooting.md)
- [GitHub Issues](https://github.com/i-rtfsc/NeuraDock/issues)
