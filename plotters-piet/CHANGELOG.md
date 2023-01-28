# Changelog

## v0.3.2 (2023-01-28)
* Bump `piet-common` to 0.6.1 to match `druid` 0.8.2.

## v0.3.1 (2022-03-02)
* Get paths to work on windows. Direct2d requires the first element of a
  path to actually be a MoveTo, it doesn't allow a LineTo.

## v0.3.0 (2022-02-28)
* Initial version. Publishing as 0.3 instead of 0.1 to follow the plotters
  convention that backends should use the same major and minor versions as
  the `plotters` and `plotters_backend` crate versions with which they are
  compatible.
