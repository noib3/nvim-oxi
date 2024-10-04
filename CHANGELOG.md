# Changelog

## [Unreleased]

### Added

- a `String::to_str()` method which returns a `&str` if the string contains
  valid UTF-8 data;

### Changed

- `nvim_oxi::api::echo` is now generic over the highlight group type instead of
  expecting a string slice;

- renamed the `lua_<Foo>` types to `<Foo>`;

## [0.5.1] - June 23 2024

### Added

- a `handle` method on `Buffer`, `Window`, and `TabPage` which returns the
  underlying handle ([#176](https://github.com/noib3/nvim-oxi/pull/176));

### Removed

- the following methods were included in Neovim 0.10 as experimental, but have
  subsequently been removed on nightly:
    * `nvim_oxi::api::SetExtmarkOptsBuilder::scoped()`
    * `nvim_oxi::api::Window::add_ns()`
    * `nvim_oxi::api::Window::get_ns()`
    * `nvim_oxi::api::Window::del_ns()`

## [0.5.0] - May 28 2024

### Added

- support for Neovim 0.10;

- the ability to return a `Result<(), T>` from the `nvim_oxi::test` macro
  ([#159](https://github.com/noib3/nvim-oxi/pull/159));

- the optional `nvim-oxi` and `cmd` attributes to the `nvim_oxi::test` macro
  ([#159](https://github.com/noib3/nvim-oxi/pull/159));

- the optional `library_path` attribute to the `nvim_oxi::test` macro
  ([#164](https://github.com/noib3/nvim-oxi/pull/164));

### Changed

- renamed the macro that marks the entrypoint of a plugin from
  `nvim_oxi::module` to `nvim_oxi::plugin`
  ([#142](https://github.com/noib3/nvim-oxi/pull/142));

- `nvim_oxi::api:echo()` now requires a 3rd parameter of type `EchoOpts`
  ([#145](https://github.com/noib3/nvim-oxi/pull/145));


### Removed

- support for Neovim 0.8;

## [0.4.2] - Jan 29 2024

## [0.4.1] - Dec 16 2023

### Added

- a new `ExtmarkVirtTextChunk` struct;

- an `Inline` variant to the `ExtmarkVirtTextPosition` enum when building for
  `neovim-nightly`;

### Changed

- the type of `ExtmarkInfos`'s `virt_text` field from
  `Option<Vec<(String, OneOrMore<String>)>>` to `Vec<ExtmarkVirtTextChunk>`;

## [0.4.0] - Dec 11 2023

[Unreleased]: https://github.com/noib3/nvim-oxi/compare/v0.5.1...HEAD
[0.5.1]: https://github.com/noib3/nvim-oxi/tree/v0.5.1
[0.5.0]: https://github.com/noib3/nvim-oxi/tree/v0.5.0
[0.4.2]: https://github.com/noib3/nvim-oxi/tree/v0.4.2
[0.4.1]: https://github.com/noib3/nvim-oxi/tree/v0.4.1
[0.4.0]: https://github.com/noib3/nvim-oxi/tree/v0.4.0
