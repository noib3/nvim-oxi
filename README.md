# :link: nvim-oxi

[![Latest version]](https://crates.io/crates/nvim-oxi)
[![Docs]](https://docs.rs/nvim-oxi)
[![MSRV]](https://www.rust-lang.org/)
[![CI]](https://github.com/noib3/nvim-oxi/actions)

[Latest version]: https://img.shields.io/crates/v/nvim-oxi.svg
[Docs]: https://docs.rs/nvim-oxi/badge.svg
[MSRV]: https://img.shields.io/badge/rustc-1.62+-brightgreen.svg?&logo=rust
[CI]: https://github.com/noib3/nvim-oxi/actions/workflows/tests.yml/badge.svg

## What

The `nvim-oxi` crate provides first-class Rust bindings to the rich API exposed
by the [Neovim](https://neovim.io) terminal text editor.

The project is mostly intended for plugin authors, although nothing's stopping
end users from writing their Neovim configs in Rust.

## How

TODO!

## Why

Why bother when Neovim already exposes its rich and powerful API through Lua?
Two main reasons:

- access to the Rust ecosystem: Lua is a great, minimal scripting language but
  can also be limiting when writing more complex plugins. In contrast, Rust is
  a fully-fledged, statically typed language with a huge ecosystem of crates
  for (de)serialization, networking, IO, green threads, etc;

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

The `test` feature flag enables the `#[nvim_oxi::test]` proc macro. This macro
replaces the regular `#[test]` annotations and can be used to test a piece of
code from within a Neovim instance using Rust's excellent testing framework.

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

Then `cargo test` will spawn a new Neovim process with an empty config, run
that code and exit. There are a couple of gotchas:

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
