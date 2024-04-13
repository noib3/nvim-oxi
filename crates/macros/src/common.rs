use proc_macro2::Span;
use syn::parse::Parse;

/// A trait implemented by key-value macro attributes.
pub(crate) trait KeyedAttribute: Parse {
    /// The key of the attribute.
    const KEY: &'static str;

    /// Returns the span of the key.
    fn key_span(&self) -> Span;
}

/// An error returned when the same attribute is specified more than once.
pub(crate) struct DuplicateError<T>(pub(crate) T);

impl<T: KeyedAttribute> From<DuplicateError<T>> for syn::Error {
    #[inline]
    fn from(DuplicateError(attr): DuplicateError<T>) -> Self {
        struct ErrorMsg(&'static str);

        impl core::fmt::Display for ErrorMsg {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "duplicate attribute: `{}`", self.0)
            }
        }

        syn::Error::new(attr.key_span(), ErrorMsg(T::KEY))
    }
}
