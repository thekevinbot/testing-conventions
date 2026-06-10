# Migrations

Upgrade notes for breaking changes. New entries go under `## Unreleased`.
On release, the section is renamed to `## v<OLD> → v<NEW>`.

Each entry has five sections, in order:

1. **Summary** — one paragraph: what changed and why.
2. **Required changes** — before/after for public API. "None" if purely additive.
3. **Deprecations removed** — anything previously warned about that's now gone.
4. **Behavior changes without code changes** — same API, different runtime behavior.
5. **Verification** — commands that confirm the upgrade worked, with expected output.

## Unreleased

### Summary

Adds the `config` module: a `Config` schema mirroring the README's configuration
tables, plus `load_config()`, which reads one TOML file into it and validates
the config itself (the self-guard) — unknown keys and malformed TOML are
rejected rather than silently accepted. Purely additive; nothing consumes the
parsed config yet.

### Required changes

None. New, additive API: `testing_conventions::config::{Config, load_config}`.

### Deprecations removed

None.

### Behavior changes without code changes

None.

### Verification

```
cd packages/rust && cargo test --test config_loader
```

Expected: the loader's integration tests pass — the canonical config loads into
memory, and unknown-key, malformed, and missing-file configs are rejected.
