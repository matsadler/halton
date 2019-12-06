# Changelog

## [0.2.1] - 2019-12-05
### Changed
- Accidentally public helper function hidden in docs and marked deprecated

## [0.2.0] - 2019-12-04
### Added
- `#![no_std]` attribute to build without the standard library
- `halton::number` function to generate a single Halton number
- `halton::Sequence` can be cloned
- `Iterator::size_hint` for `halton::Sequence` returns accurate size
- Optimised `Iterator::nth` (and therefore also `Iterator::skip`), `Iterator::count`,
  and `Iterator::last` for `halton::Sequence`
- Implemented `ExactSizeIterator` for `halton::Sequence`

### Changed
- Upgraded to Rust 2018 edition

### Removed
- `halton::Sequence::skip` constructor, instead use `halton::Sequence::new().skip()`

### Fixed
- Correct upper limit of the sequence

## [0.1.0] - 2018-04-15
### Added
- `halton::Sequence` iterator over Halton numbers