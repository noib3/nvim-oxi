# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a
Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ObjectKind` enum is exposed
  ([ac818c4](https://github.com/noib3/nvim-oxi/commit/ac818c46b897bd73e1340ea15a641f858259a22b));

- added `ToFunction` trait which allows to accept both rust closures and
  `Function`s ([3002b0b](https://github.com/noib3/nvim-oxi/commit/3002b0bf37c1107567f0eaf0b1d5b2bb10124a08));

- `LuaPoppable` and `LuaPushable` traits are exposed
  ([6cb5ebd](https://github.com/noib3/nvim-oxi/commit/6cb5ebdfce7f61350985a9a5683cfd6fc4dcb9b6));

### Changed

- `#[nvim_oxi::module]` now takes a function pointer instead of closure
  ([f2da6d0](https://github.com/noib3/nvim-oxi/commit/f2da6d01d1b4bae7c66e3378e77bfe755e71600f));

[unreleased]: https://github.com/noib3/nvim-oxi/compare/v0.1.3...HEAD
