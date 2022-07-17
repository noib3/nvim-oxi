# Examples

There are currently only 3 example plugins, with more to be added as requested.
Roughly in order of which one you should go through first:

- `calc`: shows how to expose your plugin's core functions to Lua;
- `api`: shows how to use the `api` module to create commands, set keymaps and
  manipulate floating windows;
- `mechanic`: shows how to deserialize Lua tables into Rust objects using
  [`serde`](https://serde.rs).

The examples also include some snippets of Lua code that you can use to test
the plugins from Neovim once you've compiled and loaded them following the
steps below.

## Crate setup

The first step is to create a new library crate with `cargo new --lib
{your_plugin}` and edit the generated `Cargo.toml` to include:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
nvim-oxi = "0.1"
```

Next, in `lib.rs` we'll annotate the entry point of the plugin with the
`#[nvim_oxi::module]` macro:

```rust
// lib.rs
use nvim_oxi as oxi;

#[oxi::module]
fn foo() -> oxi::Result<i32> {
    Ok(42)
}
```

macOS users will also need to set a few linker arguments to tell the Rust
linker that the FFI functions `nvim-oxi` links to will only be available at
runtime. A possible way to do this is to create a `.cargo/config` file with the
following content:

```toml
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```

After building the crate with `cargo build {--release}`, cargo will place the
compiled artifacts in `target/debug` or `target/release` depending on whether
you built a debug or release version of the crate. If the package name
specified in `Cargo.toml` is `"foo"`, the library will be named:

  - `libfoo.so` on Linux;
  - `libfoo.dylib` on macOS;
  - `foo.dll` on Windows.

Next, we need to tell Neovim where to load the plugin from. Create a new
directory named `lua` and place the compiled library inside it, renaming it to

  - `foo.so` on Linux;
  - `foo.so` on macOS;
  - `foo.dll` on Windows (i.e. no renaming).

Now open Neovim and add *the parent directory* of `lua` to the
[runtimepath](https://neovim.io/doc/user/options.html#'runtimepath'), for
example with `:set rtp+=~/foobar`, assuming `lua` is in `~/foobar/lua`.

And we're done. You can now call the `require` function to load the plugin just
like any other Lua plugin, which will return the output of the `foo()` function
defined in `lib.rs`:

```lua
print(require("foo")) -- prints `42`
```
