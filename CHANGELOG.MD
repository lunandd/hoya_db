# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0-dev] - 2022-07-14
### Added
- Added the `db` crate
- Added the `hoya` crate
- Added an interpreter
- Added 3 new types to hoya::parser::parser
    - `EasyStreamError<'a>`
    - `EasyStreamOk<'a>`
    - `ParserResult<'a>`
- Added the `Unit` type

### Changed
- Renamed the `thorn` crate to `hoya`
- Renamed the project to HoyaDB
- Renamed the `DB` struct to `Database`
- Made the `hoya::db::db` a separate crate called `db`
- Made all methods of the `Database` struct that were mutable immutable, because they didn't need to be mutable
- List elements are now separated by whitespace rather than commas
- Changed the return type of the `hoya::parser::parser::parse` function to `ParserResult`

## [0.2.0-dev] - 2022-07-27
### Added
- `store` function in the `Database` struct
- `load` function in the `Database` struct
- `store` function in the Interpreter
- `load` function in the Interpreter

## [1.0.0-dev] - 2022-07-27
### Fixed
- #1

### Changed
- Renamed the `synthesize` function to `single_return_synthesize`

### Added
- `synthesize` function

[0.1.0-dev]: https://github.com/lunandd/hoya_db/commit/34959951221155102d86ef2115c6ce9f55efcdf3
[0.2.0-dev]: https://github.com/lunandd/hoya_db/commit/0021b89006bbe51a9cbd02795165e084561b68b2
