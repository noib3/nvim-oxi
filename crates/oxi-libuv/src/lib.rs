mod r#async;
mod error;
mod handle;
mod r#loop;
mod timer;

pub use error::Error;
use error::Result;
use handle::Handle;
#[cfg(feature = "use-1-44")]
use libuv_sys2_1_44 as libuv_sys2;
#[cfg(feature = "use-1-46")]
use libuv_sys2_1_46 as libuv_sys2;
pub use r#async::AsyncHandle;
pub use r#loop::init;
use r#loop::with_loop;
pub use timer::TimerHandle;
