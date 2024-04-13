use proc_macro2::Span;
use syn::parse::Parse;

/// A trait implemented by key-value macro attributes.
pub(crate) trait KeyedAttribute: Parse {
    /// The key of the attribute.
    const KEY: &'static str;

    /// The value of the attribute.
    type Value: Parse;

    /// Returns the span of the key.
    fn key_span(&self) -> Span;
}

/// A parses for `key = value` attributes.
///
/// The [`Parse`] implementation of this struct is guaranteed to leave the
/// input cursor unchanged if the `key` doesn't match the expected value.
///
/// This makes it possible to parse the same input multiple times with
/// different attributes until one of them parses successfully.
pub(crate) struct Keyed<T: KeyedAttribute> {
    pub(crate) value: T::Value,
}

impl<T: KeyedAttribute> Parse for Keyed<T> {
    #[inline]
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // First, lookahead to see if the key is ours.
        if input.fork().parse::<syn::Ident>()? != T::KEY {
            return Err(input.error("invalid attribute"));
        }

        let _key = input.parse::<syn::Ident>().expect("just checked");
        let _eq = input.parse::<syn::Token![=]>()?;
        let value = input.parse::<T::Value>()?;
        Ok(Self { value })
    }
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
