# 🔗 nvim-oxi

[![Latest version]](https://crates.io/crates/nvim-oxi)
[![CI]](https://github.com/noib3/nvim-oxi/actions)
[![Docs]](https://docs.rs/nvim-oxi)

[Latest version]: https://img.shields.io/crates/v/nvim-oxi.svg
[CI]: https://github.com/noib3/nvim-oxi/actions/workflows/ci.yaml/badge.svg
[Docs]: https://docs.rs/nvim-oxi/badge.svg

nvim-oxi provides safe and idiomatic Rust bindings to the rich API exposed by
the [Neovim](https://neovim.io) text editor.

The project is mostly intended for plugin authors, although nothing's
stopping end users from writing their Neovim configs in Rust.

## How

The traditional way to write Neovim plugins in languages other than the
"builtin" ones, i.e. Vimscript or Lua, is via [RPC
channels](https://neovim.io/doc/user/api.html#RPC). This approach comes with a
few limitations mostly due to having to (de)serialize everything to
MessagePack-encoded messages, prohibiting things like attaching callbacks to
keymaps or scheduling functions.

nvim-oxi takes a different approach. It leverages Rust's foreign function
interface (FFI) support to hook straight into the Neovim C code, allowing
feature parity with "in process" plugins while also avoiding the need for an
extra IO layer.

[This
thread](https://neovim.discourse.group/t/calling-neovim-internal-functions-with-luajit-ffi-and-rust)
on the Neovim discourse goes into a bit more detail for anyone who's
interested.

## Why

Why bother when Neovim already has Lua as a first-class citizen? Mainly two
reasons:

- access to the Rust ecosystem: Lua is a great, minimal scripting language but
  can also be limiting when writing more complex plugins. In contrast Rust is
  a fully-fledged, statically typed language with a huge ecosystem of crates
  for (de)serialization, networking, IO, green threads, etc;

- nvim-oxi provides a fully typed API: everything from optional function
  fields to callback arguments is checked at compile-time. This allows plugin
  authors to spend less time reading through the help docs and more time
  iterating via `cargo check`s.

## Examples

The [examples](https://github.com/noib3/nvim-oxi/tree/main/examples)
directory contains several examples of how to use nvim-oxi. It also contains
instructions on how to setup your Rust crate, where to place the compiled
artifacts and how to load the final plugin from Neovim.

If you're still not sure about something feel free to open a new issue and I
might add a new example documenting your use case (if it can be done).

## Testing

Turning on the `test` feature enables `#[nvim_oxi::test]`, which replaces the
regular `#[test]` macro and allows you to test a piece of code from within a
Neovim instance using Rust's testing framework.

For example:

```rust
use nvim_oxi::api;

#[nvim_oxi::test]
fn set_get_del_var() {
    api::set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), api::get_var("foo"));
    assert_eq!(Ok(()), api::del_var("foo"));
}
```

When `cargo test` is executed, the generated code will spawn a new Neovim
process with the `nvim` binary in your `$PATH`, test your code, and exit.

There's a gotcha: you can't have two tests with the same name in the same
crate, even if they belong to different modules. For example, this won't work:

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

Note that all integration tests must live inside a separate `cdylib` crate with
the following build script:

```rust
// build.rs
fn main() -> Result<(), nvim_oxi::tests::BuildError> {
    nvim_oxi::tests::build()
}
```
