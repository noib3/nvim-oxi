# Changelog

## [Unreleased]

### Added

- a new `ExtmarkVirtTextChunk` struct;

### Changed

- the type of `ExtmarkInfos`'s `virt_text` field from
  `Option<Vec<(String, OneOrMore<String>)>>` to `Vec<ExtmarkVirtTextChunk>`;

## [0.4.0] - Dec 11 2023

[Unreleased]: https://github.com/noib3/nvim-oxi/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/noib3/nvim-oxi/tree/v0.4.0
