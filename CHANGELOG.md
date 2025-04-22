# Changelog

All notable changes to the WFL project will be documented in this file.

## [v0.0.0-nightly+memfix] - 2025-04-22

### Fixed
- Eliminated memory leaks by using weak references for captured environments
- Optimized parser memory usage by reducing unnecessary cloning and pre-allocating collections
- Implemented string interning for identifiers to reduce memory usage
- Added SafeDebug trait with truncation limits to prevent memory explosions in debug output
- Improved call stack hygiene to prevent lingering references

### Added
- Memory leak regression tests to prevent future regressions
- Heaptrack integration for memory profiling
- Documentation for memory management strategies

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

### Changed
- Updated build system to support cross-platform compilation
