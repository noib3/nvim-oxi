use std::iter::FusedIterator;

/// A super trait of most common traits implemented on iterators.
pub trait SuperIterator<I>:
    Iterator<Item = I> + ExactSizeIterator + DoubleEndedIterator + FusedIterator
{
}

impl<I, T> SuperIterator<I> for T where
    T: Iterator<Item = I>
        + ExactSizeIterator
        + DoubleEndedIterator
        + FusedIterator
{
}
