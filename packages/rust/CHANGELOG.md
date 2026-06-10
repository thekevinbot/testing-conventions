# Changelog

All notable changes to this package are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## Unreleased

### Added

- `config` module — `load_config()` parses one TOML config file into the new
  in-memory `Config` schema (the `[python]` / `[typescript]` / `[rust]` /
  `[rules]` tables from the README) and self-validates on load: unknown keys
  and malformed TOML are rejected. The parsed config is not consumed yet. (#12)

### Changed

### Deprecated

### Removed

### Fixed
