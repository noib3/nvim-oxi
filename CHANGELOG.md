# Changelog

## [Unreleased]

## [0.4.1] - Dec 16 2023

### Added

- a new `ExtmarkVirtTextChunk` struct;

- an `Inline` variant to the `ExtmarkVirtTextPosition` enum when building for
  `neovim-nightly`;

### Changed

- the type of `ExtmarkInfos`'s `virt_text` field from
  `Option<Vec<(String, OneOrMore<String>)>>` to `Vec<ExtmarkVirtTextChunk>`;

## [0.4.0] - Dec 11 2023

[Unreleased]: https://github.com/noib3/nvim-oxi/compare/v0.4.1...HEAD
[0.4.1]: https://github.com/noib3/nvim-oxi/tree/v0.4.1
[0.4.0]: https://github.com/noib3/nvim-oxi/tree/v0.4.0
