use core::convert::Infallible;

/// A trait for types that can be converted into a `Result`.
///
/// This trait can be used to be generic over functions that can return either
/// a `T` or a `Result<T, E>`.
///
/// # Examples
///
/// ```
/// # use nvim_oxi_luajit::IntoResult;
/// fn double<F: Fn() -> R, R: IntoResult<usize>>(
///     f: F,
/// ) -> Result<usize, R::Error> {
///     f().into_result().map(|x| x * 2)
/// }
///
/// # fn main() {
/// // `double` takes a closure whose return type is generic over `IntoResult`,
/// // so we don't have to return `Ok(21)`.
/// assert_eq!(double(|| 21), Ok(42));
/// # }
/// ```
pub trait IntoResult<T> {
    /// The error type in the returned `Result`.
    type Error;

    /// Converts the value into a `Result`.
    fn into_result(self) -> Result<T, Self::Error>;
}

impl<T> IntoResult<T> for T {
    type Error = Infallible;

    #[inline]
    fn into_result(self) -> Result<T, Self::Error> {
        Ok(self)
    }
}

impl<T, E> IntoResult<T> for Result<T, E> {
    type Error = E;

    #[inline]
    fn into_result(self) -> Result<T, Self::Error> {
        self
    }
}
