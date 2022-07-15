# :link: nvim-oxi

[![Latest version]](https://crates.io/crates/nvim-oxi)
[![Docs]](https://docs.rs/nvim-oxi)
[![MSRV]](https://www.rust-lang.org/)
[![CI]](https://github.com/noib3/nvim-oxi/actions)

[CI]: https://github.com/noib3/nvim-oxi/actions/workflows/tests.yml/badge.svg
[Docs]: https://docs.rs/nvim-oxi/badge.svg
[Latest version]: https://img.shields.io/crates/v/nvim-oxi.svg
[MSRV]: https://img.shields.io/badge/Rust-1.62+-brightgreen.svg?&logo=rust

## What

Morbi eu mauris sapien. Ut at lorem sapien. Integer sollicitudin leo urna, at
tempus lacus facilisis nec. Aliquam auctor tincidunt erat in consequat.
Maecenas ultricies bibendum dolor, et dignissim elit iaculis vitae. Curabitur
diam odio, sollicitudin ac lectus ut, interdum blandit diam. Mauris mattis ex
efficitur lacus sagittis molestie. Integer tempus purus eget maximus egestas.
Cras ut bibendum lorem. Pellentesque non vehicula ante. Integer nec scelerisque
mi, a feugiat libero.

## How

Morbi eu mauris sapien. Ut at lorem sapien. Integer sollicitudin leo urna, at
tempus lacus facilisis nec. Aliquam auctor tincidunt erat in consequat.
Maecenas ultricies bibendum dolor, et dignissim elit iaculis vitae. Curabitur
diam odio, sollicitudin ac lectus ut, interdum blandit diam. Mauris mattis ex
efficitur lacus sagittis molestie. Integer tempus purus eget maximus egestas.
Cras ut bibendum lorem. Pellentesque non vehicula ante. Integer nec scelerisque
mi, a feugiat libero.

## Why

Why bother when Neovim already exposes its rich and powerful API through Lua?
Two main reasons:

- access to the Rust ecosystem: Lua is a great, minimal scripting language but
  can also be limiting when writing more complex plugins. In contrast, Rust is
  a fully-fledged language with a huge ecosystem of crates for
  (de)serialization, networking, IO, green threads, etc;

- `nvim-oxi` provides a fully typed API: everything from optional function
  fields to callback arguments is checked at compile-time. This allows plugin
  authors to spend less time reading through the help docs and more time
  iterating via `cargo check`s.

<!--
Lastly, there is possibly some performance to be gained by directly interacting
with the C code instead of serializing and deserializing MessagePack messages
like RPC plugins do. However the improvements are likely to not be noticeable
for most common tasks, and just having a
-->

## Examples

The [examples](https://github.com/noib3/nvim-oxi/tree/master/examples)
directory contains several examples of how to use `nvim-oxi`. It also contains
instructions on how to setup your Rust crate, where to place the compiled
artifacts and how to load the final plugin from Neovim.

If you're still not sure about something feel free to open a new issue and I
might add a new example documenting your use case (if it can be done).

## Testing

The `test` feature flag enables the `#[nvim_oxi::test]` proc macro which you
can use instead of the regular `#[test]` annotation to test a piece of code
using Rust's excellent testing framework.

For example:

```rust
use nvim_oxi::{self as oxi, api};

#[oxi::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}
```

Then `cargo test` will test that code from within a new Neovim instance with an
empty config. There are a couple of gotchas:

- after changing a piece of code, `cargo build` has to be run before you can
  test that with `cargo test`;

- you cannot have two test functions with the same name, even if belonging to
  different modules. For example this won't work:

```rust
mod a {
    #[nvim_oxi::test]
    fn foo() {}
}

mod b {
    #[nvim_oxi::test]
    fn foo() {}
}
```
