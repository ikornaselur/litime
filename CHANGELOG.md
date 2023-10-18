# Changelog

## [3.0.4](https://github.com/ikornaselur/litime/compare/v3.0.3...v3.0.4) (2023-10-18)


### Miscellaneous

* Add linux arm64 build target ([#60](https://github.com/ikornaselur/litime/issues/60)) ([1858b2b](https://github.com/ikornaselur/litime/commit/1858b2bfe2764986be10ac568b0ca3d1913caad5))

## [3.0.3](https://github.com/ikornaselur/litime/compare/v3.0.2...v3.0.3) (2023-10-18)


### Miscellaneous

* **CI:** Replace deprecated actions-rs actions  ([#59](https://github.com/ikornaselur/litime/issues/59)) ([f011aec](https://github.com/ikornaselur/litime/commit/f011aecaa49f167944895b6d972ba7deb26f44a3))
* Fix lint issue in build.rs ([#56](https://github.com/ikornaselur/litime/issues/56)) ([a28da38](https://github.com/ikornaselur/litime/commit/a28da38fd66f7a002271193d8b50363aaa270d8a))


### Dependencies

* bump rustix from 0.38.15 to 0.38.19 ([#58](https://github.com/ikornaselur/litime/issues/58)) ([d8af610](https://github.com/ikornaselur/litime/commit/d8af610d193b7ef8c909fde2497b69d982b0844a))
* bump serde from 1.0.188 to 1.0.189 ([#54](https://github.com/ikornaselur/litime/issues/54)) ([4f3fc38](https://github.com/ikornaselur/litime/commit/4f3fc38bab857dd787293e096e60f6b35d319dd0))
* bump time from 0.3.29 to 0.3.30 ([#55](https://github.com/ikornaselur/litime/issues/55)) ([82f1846](https://github.com/ikornaselur/litime/commit/82f1846baddaf7b9d42300b733217aa25c1243db))

## [3.0.2](https://github.com/ikornaselur/litime/compare/v3.0.1...v3.0.2) (2023-09-29)


### Miscellaneous

* Add dependabot ([#49](https://github.com/ikornaselur/litime/issues/49)) ([ee377a0](https://github.com/ikornaselur/litime/commit/ee377a0cdc7acf7a9adb0c7c39ad883bb4490cd8))


### Dependencies

* bump clap from 4.4.5 to 4.4.6 ([#51](https://github.com/ikornaselur/litime/issues/51)) ([6773ed0](https://github.com/ikornaselur/litime/commit/6773ed0b65b6e577c78d7a67ba3e1453a3fd1818))
* **github-actions:** bump actions/checkout from 2 to 4 ([#50](https://github.com/ikornaselur/litime/issues/50)) ([47c7ba3](https://github.com/ikornaselur/litime/commit/47c7ba3da9fbf4d57852545793b46a32b1895f24))
* Replace termsize with terminal_size, which is maintained ([#53](https://github.com/ikornaselur/litime/issues/53)) ([bfee3d5](https://github.com/ikornaselur/litime/commit/bfee3d5fb64460a1759605d2b186d61d002a017e))

## [3.0.1](https://github.com/ikornaselur/litime/compare/v3.0.0...v3.0.1) (2023-09-28)


### Bug Fixes

* Change the default colours to be intense instead of bold ([813373f](https://github.com/ikornaselur/litime/commit/813373f81805c5a36ba83e8fc647188c92fb52f9))

## [3.0.0](https://github.com/ikornaselur/litime/compare/v2.0.0...v3.0.0) (2023-09-28)


### ⚠ BREAKING CHANGES

* Use termcolor to render color and style for better Windows support  ([#47](https://github.com/ikornaselur/litime/issues/47))

### Features

* Use termcolor to render color and style for better Windows support  ([#47](https://github.com/ikornaselur/litime/issues/47)) ([63ee728](https://github.com/ikornaselur/litime/commit/63ee728277acbe742ec7df4b2ef750f5c9996313))


### Dependencies

* Remove unused features ([#46](https://github.com/ikornaselur/litime/issues/46)) ([a2389c5](https://github.com/ikornaselur/litime/commit/a2389c5ab2c53a79bb1ed6141ddb5779d2b47751))
* Replace rand with a pseudo random picker based on current timestamp ([#44](https://github.com/ikornaselur/litime/issues/44)) ([76c6700](https://github.com/ikornaselur/litime/commit/76c6700ff59c3ef6d7cfa2528836a493d42edb3e))

## [2.0.0](https://github.com/ikornaselur/litime/compare/v1.3.0...v2.0.0) (2023-09-27)


### ⚠ BREAKING CHANGES

* Add support for using styles and colour as formatting ([#41](https://github.com/ikornaselur/litime/issues/41))

### Features

* Add support for using styles and colour as formatting ([#41](https://github.com/ikornaselur/litime/issues/41)) ([baa4fb5](https://github.com/ikornaselur/litime/commit/baa4fb5e9a4c398a3c2ccc152910e5e8c661a0a5))

## [1.3.0](https://github.com/ikornaselur/litime/compare/v1.2.0...v1.3.0) (2023-09-27)


### Features

* Build the minutes directly with a build.rs script ([#39](https://github.com/ikornaselur/litime/issues/39)) ([246244f](https://github.com/ikornaselur/litime/commit/246244f6b19fa9415bea5a16aa1bb379ec709b11))

## [1.2.0](https://github.com/ikornaselur/litime/compare/1.1.0...v1.2.0) (2023-09-26)


### Features

* Update quotes ([#37](https://github.com/ikornaselur/litime/issues/37)) ([7417bd0](https://github.com/ikornaselur/litime/commit/7417bd0a91da436b491ae6be2578dbf7a99fd712))


### Miscellaneous

* Add release-please ([#34](https://github.com/ikornaselur/litime/issues/34)) ([8a22094](https://github.com/ikornaselur/litime/commit/8a220944ccbaa9a1fabfeda82b64ebea9a011c5c))


### Dependencies

* Update dependencies ([#36](https://github.com/ikornaselur/litime/issues/36)) ([a16a491](https://github.com/ikornaselur/litime/commit/a16a491affda80d290a3608b278283554f539d7e))
