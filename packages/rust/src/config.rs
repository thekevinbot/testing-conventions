//! The testing-conventions config schema and loader.
//!
//! One config file is read into the in-memory [`Config`] below. The loader is
//! responsible for parsing *and* validating the config itself (the "self-guard"
//! from issue #12): a malformed or unknown-key config is an error, never a
//! silently-accepted default.
//!
//! Nothing consumes [`Config`] yet — this module only turns the file on disk
//! into the in-memory structure.

use std::path::Path;

use anyhow::Result;

/// A fully-parsed testing-conventions config file.
///
/// Mirrors the `[python]` / `[typescript]` / `[rust]` / `[rules]` tables in the
/// README's "Configuration" section. Each language table is optional so a repo
/// can configure only the languages it ships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub python: Option<PythonConfig>,
    pub typescript: Option<TypeScriptConfig>,
    pub rust: Option<RustConfig>,
    pub rules: Option<Rules>,
}

/// The `[python]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonConfig {
    pub unit: UnitGlob,
    pub integration: TestDir,
    pub e2e: TestDir,
    pub coverage: PythonCoverage,
}

/// The `[typescript]` table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScriptConfig {
    pub unit: UnitGlob,
    pub integration: TestDir,
    pub e2e: TestDir,
    pub coverage: TypeScriptCoverage,
}

/// The `[rust]` table. Units are inline `#[cfg(test)]`, so only coverage is
/// configured here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustConfig {
    pub coverage: RustCoverage,
}

/// `unit.glob` — where a language's colocated unit tests live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitGlob {
    pub glob: String,
}

/// `integration.dir` / `e2e.dir` — the folder a non-unit suite lives in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestDir {
    pub dir: String,
}

/// `[python].coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonCoverage {
    pub branch: bool,
    pub fail_under: u8,
}

/// `[typescript].coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScriptCoverage {
    pub lines: u8,
    pub branches: u8,
    pub functions: u8,
    pub statements: u8,
}

/// `[rust].coverage`. Branch coverage is still experimental, so only
/// regions/lines are configurable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustCoverage {
    pub regions: u8,
    pub lines: u8,
}

/// The `[rules]` table — the cross-language knobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rules {
    pub unit: UnitRules,
    pub integration: IntegrationRules,
    pub coverage: CoverageRules,
}

/// `[rules].unit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitRules {
    pub isolation: String,
}

/// `[rules].integration`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrationRules {
    pub external: String,
    pub whitelist: Vec<String>,
}

/// `[rules].coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageRules {
    pub floor: String,
}

/// Read one config file at `path` into a [`Config`], validating it on the way
/// (unknown keys, wrong types, and malformed TOML are errors — the self-guard).
///
/// Not implemented yet: the integration tests in `tests/config_loader.rs` are
/// written red first (issue #12), and this stub keeps the crate compiling so
/// the rest of CI stays green. Replacing the `todo!` with a real parser is the
/// next step.
pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref();
    todo!("parse + self-validate the config at {path:?}; tracked in issue #12")
}
