use std::{marker::PhantomData, mem::ManuallyDrop};

/// A non-owning value for lifetime 'a.
///
/// It doesn't drop the value.
/// Used for ffi function that accept data by value, but don't destroy or move out of it.
/// This is garaunteed to have same layout as T
#[repr(transparent)]
pub struct NonOwning<'a, T> {
    // never drop the T
    // field not being used here, is in use for repr(transparent)
    #[allow(unused)]
    inner: ManuallyDrop<T>,
    lf: PhantomData<&'a ()>,
}

impl<'a, T> NonOwning<'a, T> {
    pub unsafe fn new(value: T) -> Self {
        Self { inner: ManuallyDrop::new(value), lf: PhantomData }
    }
}
