use crate::luajit::{Function, Userdata};

pub struct AsyncHandle {
    handle: Userdata,
}

impl AsyncHandle {
    pub fn new<Cb>(callback: Cb) -> Self
    where
        Cb: FnMut() -> () + 'static,
    {
        let callback = Function::from_fn_mut(callback);
        let handle = luv!(new_async, callback, Userdata);
        Self { handle }
    }

    pub fn send(&self) {
        uv!(async_send, self.handle);
    }
}
