# Examples

## [`api`](./api.rs)

Shows how to use the `api` module to create commands, set keymaps and
manipulate floating windows.

## [`calc`](./calc.rs)

Shows how to expose your plugin's core functions to Lua.

```lua
local calc = require("calc")

-- All the following commands will print `42` in the Neovim message area.

print(calc.add(1, 41))
print(calc.multiply(2, 21))

print(calc.compute(function(a, b) return a + b; end, 1, 41))
print(calc.compute(function(a, b) return a * b; end, 2, 21))
```

## [`mechanic`](./mechanic.rs)

Shows how to deserialize Lua tables into Rust objects using
[`serde`](https://serde.rs).

```lua
local mechanic = require("mechanic")

local fixed = mechanic.fix({
  manufacturer = "Tesla",
  miles = 69420,
  works = false,
  problem = "kills_pedestrians",
})

assert(fixed.works)
assert(fixed.problem == nil)
```

## [`mlua`](./mlua.rs)

Shows how to integrate `nvim-oxi` with the
[`mlua`](https://github.com/khvzak/mlua) crate.

## [`libuv`](./libuv.rs)

Shows how to use the `nvim_oxi::libuv` module to trigger a callback registered
on the Neovim thread from other threads.

# Crate setup

The first step is to create a new library crate with `cargo new --lib
{your_plugin}` and edit the generated `Cargo.toml` to include:

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
nvim-oxi = "0.3"
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
