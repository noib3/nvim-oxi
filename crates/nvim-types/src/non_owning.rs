use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// A non-owning value for lifetime `'a`.
///
/// Used for FFI functions that accept data by value, but don't destroy or move
/// out of it. This is guaranteed to have the same layout as `T`.
#[doc(hidden)]
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

impl<'a, T> fmt::Debug for NonOwning<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self._inner.fmt(f)
    }
}

impl<'a, T> Default for NonOwning<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Self { _inner: ManuallyDrop::new(T::default()), _lt: PhantomData }
    }
}
