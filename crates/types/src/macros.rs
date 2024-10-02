/// Same as [`format!`] but creates an [`nvim_oxi::String`](crate::String).
#[macro_export]
macro_rules! string {
    ($($tt:tt)*) => {{
        let mut w = $crate::StringBuilder::new();
        ::core::fmt::Write::write_fmt(&mut w, format_args!($($tt)*))
            .expect("a formatting trait implementation returned an error");
        w.finish()
    }};
}
