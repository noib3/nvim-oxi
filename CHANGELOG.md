# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ObjectKind` enum is exposed
  ([ac818c4](https://github.com/noib3/nvim-oxi/commit/ac818c46b897bd73e1340ea15a641f858259a22b))

- added `ToFunction` trait which allows to accept both rust closures and
  `Function`s ([3002b0b](https://github.com/noib3/nvim-oxi/commit/3002b0bf37c1107567f0eaf0b1d5b2bb10124a08))

- `LuaPoppable` and `LuaPushable` traits are exposed
  ([6cb5ebd](https://github.com/noib3/nvim-oxi/commit/6cb5ebdfce7f61350985a9a5683cfd6fc4dcb9b6))

- `nvim_oxi::dbg!` macro
  ([7b70c7d](https://github.com/noib3/nvim-oxi/commit/7b70c7dec325c47b8b01d9bb0e0712f0caeb92b2))

- [`mlua`](https://github.com/khvzak/mlua) integration via behind `mlua`
  feature flag
  ([71e8f28](https://github.com/noib3/nvim-oxi/commit/71e8f28ad6abdd6a4ac541ceff200a1b88b1981b))

- added `replace_keycodes` field to `nvim_oxi::opts::SetKeymapOpts` in
  `nightly` ([912385e](https://github.com/noib3/nvim-oxi/commit/912385e776a28407605a729e36d2e9ecef6fe6f8))

- added `nvim_oxi::r#loop` module behind `loop` feature flag
  ([abf70cb](https://github.com/noib3/nvim-oxi/commit/abf70cbf2f5df2e4450f7578daf9008ec2548bd0))

### Changed

- `#[nvim_oxi::module]` now takes a function pointer instead of closure
  ([f2da6d0](https://github.com/noib3/nvim-oxi/commit/f2da6d01d1b4bae7c66e3378e77bfe755e71600f))

- type of `nvim_oxi::opts::OptionInfos` from `u32` to `i64` to allow negative
  integers ([43407df](https://github.com/noib3/nvim-oxi/commit/43407dffaebb916379be25e5c68d872235368c3c))

### Fixed

- ([#51](https://github.com/noib3/nvim-oxi/issues/51)) fixed a bug that would
  cause segfaults when popping tables off the Lua stack
  ([f08099d](https://github.com/noib3/nvim-oxi/commit/f08099d689a9af5b406965e6b17fa4d07693bce1))

- ([#45](https://github.com/noib3/nvim-oxi/issues/45)) fixed a bug that would
  cause segfaults when iterating over `nvim_oxi::Array`s in `nightly` builds
  ([07b03d3](https://github.com/noib3/nvim-oxi/commit/07b03d3fb6d46862111c865779d053bad5bce0d0))

[unreleased]: https://github.com/noib3/nvim-oxi/compare/v0.1.3...HEAD
