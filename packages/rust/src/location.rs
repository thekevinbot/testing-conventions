//! Unit-test location/naming check for Python sources (issue #15).
//!
//! The convention (README "Location & Naming"; `internals/python/testing.md`):
//! a Python source file `foo.py` is unit-tested by a colocated `foo_test.py`.
//! [`missing_unit_tests`] walks a directory tree and returns every source file
//! that has no such sibling — an "orphan". Files that are themselves tests
//! (`*_test.py`) are what the check looks *for*, never subjects.

use std::path::{Path, PathBuf};

use anyhow::Result;

/// Walk `root` recursively and return every Python source file that has no
/// colocated `<stem>_test.py`, sorted for deterministic output.
///
/// A file whose stem ends in `_test` is itself a test and is never treated as a
/// subject; every other `*.py` file is a subject and must have its colocated
/// test sibling. Returns an error if the tree under `root` cannot be read.
pub fn missing_unit_tests(_root: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    todo!("issue #15: scan for *.py files missing a colocated *_test.py")
}
