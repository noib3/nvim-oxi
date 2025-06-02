use core::fmt;
use std::panic::PanicHookInfo;
use std::sync::{Arc, OnceLock};

/// The error type given to [`TestTerminator::terminate`].
///
/// The two variants of this enum represent the two ways a test can fail:
/// either by returning an error or by panicking.
#[cfg_attr(docsrs, doc(cfg(feature = "test-terminator")))]
pub enum TestFailure<'a, E> {
    /// This is used to indicate that the test failed due to an error being
    /// returned from the test function.
    Error(E),

    /// This is used to indicate that the test failed due to a panic. The
    /// [`PanicHookInfo`](std::panic::PanicHookInfo) contains information about
    /// the panic and can be obtained by calling
    /// [`set_hook`](std::panic::set_hook).
    Panic(&'a PanicHookInfo<'a>),
}

/// A handle used to terminate a test annotated by [`test`](crate::test).
///
/// The `test` macro works by turning the annotated function into its own
/// plugin, which is then loaded by Neovim and evalutated by `require`ing it
/// when the test is run, before immediately quitting.
///
/// When testing asynchronous code this can be problematic, as the test may
/// need to continue running after the generated plugin has been `require`d.
///
/// To allow for this, the test function can take a `TestTerminator` as its
/// only argument. This allows the test to be terminated asynchronously by
/// calling [`terminate`](Self::terminate).
///
/// Note that if the `TestTerminator` is dropped without first calling
/// `terminate`, the test will run forever.
#[cfg_attr(docsrs, doc(cfg(feature = "test-terminator")))]
pub struct TestTerminator {
    pub(super) handle: crate::libuv::AsyncHandle,
    pub(super) result: Arc<OnceLock<super::test_macro::TestResult>>,
}

impl TestTerminator {
    /// Terminates the test.
    ///
    /// Note that this will have no effect if [`terminate`](Self::terminate)
    /// has already been called.
    pub fn terminate<E: fmt::Debug>(
        &self,
        result: Result<(), TestFailure<'_, E>>,
    ) {
        if let Ok(()) = self.result.set(result.map_err(Into::into)) {
            self.handle.send().unwrap();
        }
    }
}
