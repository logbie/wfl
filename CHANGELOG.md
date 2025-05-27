# Changelog

All notable changes to the WFL project will be documented in this file.

## [v0.0.0-nightly+CDR3] - 2025-04-20

### Fixed
- Fixed memory leak in closures by using weak references for captured environments
- Improved debug report to return a Result and show appropriate error messages
- Hardened `.clear` REPL command against stdout failures

### Changed
- Updated documentation to clarify sequential wait-for behavior

## [Unreleased]

### Added
- Nightly build and installer pipeline for Windows, Linux, and macOS
- Automated installers: MSI for Windows, tar.gz/deb for Linux, pkg for macOS
- Skip-if-unchanged logic to avoid unnecessary builds
- Default configuration files included in installers
- Documentation for building and releasing WFL
- Configuration validation & auto-fix flags (`--configCheck` and `--configFix`) - 2025-05-19

### Changed
- Updated build system to support cross-platform compilation

### Fixed
- Fixed memory leak in closures with weak references to parent environments
- Improved file I/O with append-mode operations instead of read-modify-write
- Optimized parser memory allocations to reduce heap churn
- Fixed static analyzer incorrectly flagging variables as unused in action definitions, I/O statements, and action calls
