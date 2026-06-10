//! Integration tests for the config schema + loader (issue #12).
//!
//! Written **red first**: `load_config` is a stub, so every test here fails
//! until the loader is implemented. They pin the contract from the README's
//! "Configuration" section — one config file is read into the in-memory
//! `Config` — and the self-guard: a config that fails its own validation
//! (unknown keys, malformed TOML) is rejected rather than silently accepted.
//!
//! Per the #3 guardrail, the loader ships a clean fixture (`valid.toml`, must
//! load) and red fixtures (`unknown_key.toml` / `malformed.toml`, must fail).

use std::path::PathBuf;

use testing_conventions::config::{
    load_config, Config, CoverageRules, IntegrationRules, PythonConfig, PythonCoverage, Rules,
    RustConfig, RustCoverage, TestDir, TypeScriptConfig, TypeScriptCoverage, UnitGlob, UnitRules,
};

/// Absolute path to a file under `tests/fixtures/`.
fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// The in-memory shape we expect `valid.toml` to parse into — i.e. the README's
/// canonical config, loaded.
fn expected_valid() -> Config {
    Config {
        python: Some(PythonConfig {
            unit: UnitGlob {
                glob: "src/**/*_test.py".into(),
            },
            integration: TestDir {
                dir: "tests/integration".into(),
            },
            e2e: TestDir {
                dir: "tests/e2e".into(),
            },
            coverage: PythonCoverage {
                branch: true,
                fail_under: 100,
            },
        }),
        typescript: Some(TypeScriptConfig {
            unit: UnitGlob {
                glob: "src/**/*.test.ts".into(),
            },
            integration: TestDir {
                dir: "tests/integration".into(),
            },
            e2e: TestDir {
                dir: "tests/e2e".into(),
            },
            coverage: TypeScriptCoverage {
                lines: 100,
                branches: 100,
                functions: 100,
                statements: 100,
            },
        }),
        rust: Some(RustConfig {
            coverage: RustCoverage {
                regions: 100,
                lines: 100,
            },
        }),
        rules: Some(Rules {
            unit: UnitRules {
                isolation: "error".into(),
            },
            integration: IntegrationRules {
                external: "all".into(),
                whitelist: vec!["lodash".into(), "chrono".into()],
            },
            coverage: CoverageRules {
                floor: "no-regress".into(),
            },
        }),
    }
}

#[test]
fn loads_the_canonical_config_into_memory() {
    let config = load_config(fixture("valid.toml")).expect("the canonical config should load");
    assert_eq!(config, expected_valid());
}

#[test]
fn rejects_unknown_keys_self_guard() {
    let result = load_config(fixture("unknown_key.toml"));
    assert!(
        result.is_err(),
        "an unknown config key must be rejected (self-guard), got: {result:?}"
    );
}

#[test]
fn rejects_malformed_toml() {
    let result = load_config(fixture("malformed.toml"));
    assert!(
        result.is_err(),
        "malformed TOML must be rejected, got: {result:?}"
    );
}
