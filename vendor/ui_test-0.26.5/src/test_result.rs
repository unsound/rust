//! Various data structures used for carrying information about test success or failure

use std::sync::{atomic::AtomicBool, Arc};

use crate::{status_emitter::TestStatus, Error};
use color_eyre::eyre::Result;

/// The possible non-failure results a single test can have.
#[derive(Debug)]
pub enum TestOk {
    /// The test passed
    Ok,
    /// The test was ignored due to a rule (`//@only-*` or `//@ignore-*`)
    Ignored,
}

/// The possible results a single test can have.
pub type TestResult = Result<TestOk, Errored>;

/// Information about a test failure.
#[derive(Debug)]
pub struct Errored {
    /// Command that failed
    pub(crate) command: String,
    /// The errors that were encountered.
    pub(crate) errors: Vec<Error>,
    /// The full stderr of the test run.
    pub(crate) stderr: Vec<u8>,
    /// The full stdout of the test run.
    pub(crate) stdout: Vec<u8>,
}

impl Errored {
    /// If no command was executed for this error, use a message instead.
    pub fn new(errors: Vec<Error>, message: &str) -> Self {
        Self {
            errors,
            stderr: vec![],
            stdout: vec![],
            command: message.into(),
        }
    }

    pub(crate) fn aborted() -> Errored {
        Self::new(vec![], "aborted")
    }
}

/// Result of an actual test or sub-test (revision, fixed, run, ...) including its status.
pub struct TestRun {
    /// Actual test run output.
    pub result: TestResult,
    /// Usually created via `for_revsion` or `for_path`
    pub status: Box<dyn TestStatus>,
    /// Whether the run was aborted prematurely
    pub abort_check: Arc<AtomicBool>,
}
