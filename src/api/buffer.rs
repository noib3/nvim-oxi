use std::fmt;

pub(crate) type BufHandle = std::os::raw::c_int;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Buffer(BufHandle);

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

impl From<BufHandle> for Buffer {
    fn from(handle: BufHandle) -> Self {
        Buffer(handle)
    }
}

impl From<Buffer> for BufHandle {
    fn from(buf: Buffer) -> Self {
        buf.0
    }
}
