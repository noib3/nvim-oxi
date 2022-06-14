use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// A non-owning value for lifetime `'a`.
///
/// Used for FFI functions that accept data by value, but don't destroy or move
/// out of it. This is garaunteed to have the same layout as `T`.
#[repr(transparent)]
pub struct NonOwning<'a, T> {
    _inner: ManuallyDrop<T>,
    _lt: PhantomData<&'a ()>,
}

impl<'a, T> NonOwning<'a, T> {
    pub const fn new(value: T) -> Self {
        Self { _inner: ManuallyDrop::new(value), _lt: PhantomData }
    }
}
