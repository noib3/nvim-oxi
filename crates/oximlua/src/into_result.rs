use mlua::{ExternalError, ExternalResult};

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
/// ) -> mlua::Result<usize> {
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
    /// Converts the value into a `Result`.
    fn into_result(self) -> mlua::Result<T>;
}

impl<T> IntoResult<T> for T {
    #[inline]
    fn into_result(self) -> mlua::Result<T> {
        Ok(self)
    }
}

impl<T, E> IntoResult<T> for Result<T, E>
where
    E: ExternalError,
{
    #[inline]
    fn into_result(self) -> mlua::Result<T> {
        self.into_lua_err()
    }
}
