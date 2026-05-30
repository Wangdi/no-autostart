# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - 2026-01-XX

#### Core Features
- **Registry Scanner** - Full implementation of Windows autostart detection
  - Scan HKLM/HKCU Run and RunOnce registry keys
  - Scan startup folders (user and all users)
  - Basic Windows service detection
  - Case-insensitive process matching

- **Risk Assessment** - Dynamic risk level evaluation
  - Heuristic detection for suspicious processes (miner, hack, crack, etc.)
  - Risk levels: `safe`, `low`, `unknown`, `warning`, `dangerous`
  - Location-based risk assessment (temp folders, unusual paths)

- **Security Policy** - Comprehensive process protection
  - Protected process whitelist (system-critical processes)
  - Confirmation requirement for user applications
  - Batch operation limits
  - Risk-level based confirmation

- **Error Handling** - Structured error types
  - `thiserror`-based error definitions
  - Comprehensive error variants for all operations
  - Proper error propagation

- **Performance** - Caching and rate limiting
  - TTL-based cache for process information
  - Rate limiter for preventing excessive operations
  - Automatic cache cleanup

#### CI/CD
- GitHub Actions workflow for automated testing
  - Rust tests on Windows
  - Frontend tests on Ubuntu
  - Automated build process
  - Release draft creation

#### Developer Experience
- Comprehensive test coverage
- Module-level documentation
- Exported public API

### Changed
- `ProcessManager` now integrates `RegistryScanner` for accurate startup detection
- `detect_startup_type()` uses registry data instead of hardcoded lists
- `can_close()` respects security policy
- `assess_risk_level()` provides dynamic risk evaluation
- Improved error messages with specific error types

### Security
- Protected critical system processes from accidental termination
- Added confirmation requirements for risky operations
- Implemented batch operation limits

## [0.1.0] - 2026-01-XX

### Added
- Initial release
- Basic process listing and management
- Vue 3 + Tauri 2.0 architecture
- System tray integration
- Configuration management
- History tracking
- Local knowledge base
- Basic tests

---

## Roadmap

### Planned Features
- [ ] Digital signature verification for publisher information
- [ ] Task Scheduler API integration
- [ ] Real-time process monitoring
- [ ] Dark mode support
- [ ] Export/import configuration
- [ ] Multi-language support (i18n)
- [ ] Process grouping and categories
- [ ] Detailed process information panel
- [ ] Custom rules for auto-close
- [ ] Windows service management UI
