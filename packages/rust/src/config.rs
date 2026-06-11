//! The testing-conventions config schema and loader.
//!
//! One config file is read into the in-memory [`Config`] below. The loader
//! parses *and* validates the config itself (the "self-guard" from issue #12):
//! a malformed or unknown-key config is an error, never a silently-accepted
//! default.
//!
//! Nothing consumes [`Config`] yet — this module only turns the file on disk
//! into the in-memory structure.

use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

/// A fully-parsed testing-conventions config file.
///
/// Holds the per-language coverage thresholds — the `[python]` / `[typescript]`
/// / `[rust]` tables from the README's "Configuration" section. Each table is
/// optional so a repo can configure only the languages it ships. Test locations
/// follow convention, not config, so there are no location keys here.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub python: Option<PythonConfig>,
    pub typescript: Option<TypeScriptConfig>,
    pub rust: Option<RustConfig>,
}

/// The `[python]` table.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PythonConfig {
    pub coverage: PythonCoverage,
}

/// The `[typescript]` table.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeScriptConfig {
    pub coverage: TypeScriptCoverage,
}

/// The `[rust]` table.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustConfig {
    pub coverage: RustCoverage,
}

/// `[python].coverage`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PythonCoverage {
    pub branch: bool,
    pub fail_under: u8,
}

/// `[typescript].coverage`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeScriptCoverage {
    pub lines: u8,
    pub branches: u8,
    pub functions: u8,
    pub statements: u8,
}

/// `[rust].coverage`. Branch coverage is still experimental, so only
/// regions/lines are configurable.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RustCoverage {
    pub regions: u8,
    pub lines: u8,
}

/// Read one config file at `path` into a [`Config`], validating it on the way.
///
/// The validation is the config's self-guard: `serde`'s `deny_unknown_fields`
/// rejects keys that aren't part of the schema, missing required keys and
/// wrong-typed values are type errors, and malformed TOML fails to parse. Any
/// of these surfaces as an `Err` rather than a silently-accepted default.
pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let path = path.as_ref();
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("reading config file `{}`", path.display()))?;
    toml::from_str(&contents).with_context(|| format!("parsing config file `{}`", path.display()))
}
