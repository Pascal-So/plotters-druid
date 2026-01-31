# Changelog

## v0.3.4 (2026-01-31)
* Expand `piet-common` dependency to cover the full range `>=0.4, <0.9`. Note
  that this might not cover pre-releases, therefore you still might have to
  reach for `plotters-piet` v0.3.3 when you need exactly 0.7.0-cairo18.

## v0.3.3 (2026-01-30)
* Bump `piet-common` to 0.7.0-cairo18.

  0.7.0-cairo18 is a pre-release of `piet-common` 0.7.0. Setting the dependency
  to the pre-release allows users of `plotters-piet` to use both the pre-release
  and full release of `piet-common` 0.7.0. This is useful because the current
  `druid` main branch also depends on that pre-release.

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
