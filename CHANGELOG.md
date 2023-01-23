# Changelog

## [0.6.2](https://github.com/koesterlab/alignoth/compare/v0.6.1...v0.6.2) (2023-01-23)


### Bug Fixes

* Remove potential risk of unplaced reads for bam files with very high density ([#63](https://github.com/koesterlab/alignoth/issues/63)) ([b84c9fc](https://github.com/koesterlab/alignoth/commit/b84c9fc52a47143e78071d81f636a9e69554deff))

## [0.6.1](https://github.com/koesterlab/alignoth/compare/v0.6.0...v0.6.1) (2023-01-16)


### Bug Fixes

* Improve error message on missing when bam header doesn't contain given target ([#59](https://github.com/koesterlab/alignoth/issues/59)) ([9bc3f86](https://github.com/koesterlab/alignoth/commit/9bc3f868657cec5998f6268f30656555b8b8893c))

## [0.6.0](https://github.com/koesterlab/alignoth/compare/v0.5.3...v0.6.0) (2022-12-12)


### Features

* Improve identification of base positions ([#48](https://github.com/koesterlab/alignoth/issues/48)) ([5464a3f](https://github.com/koesterlab/alignoth/commit/5464a3f9e9147533010b768705cb52a129814e92))

## [0.5.3](https://github.com/koesterlab/alignoth/compare/v0.5.2...v0.5.3) (2022-12-12)


### Bug Fixes

* Fix highlighting offset ([#42](https://github.com/koesterlab/alignoth/issues/42)) ([1ae9c68](https://github.com/koesterlab/alignoth/commit/1ae9c68cde1c47ddc9411aa9579a1ef97a42f1da))
* Improve error message when bam header doesn't contain given target ([#47](https://github.com/koesterlab/alignoth/issues/47)) ([25df0fd](https://github.com/koesterlab/alignoth/commit/25df0fdba6cb70fb1d17e28123e3b11b77f0f7b9))

## [0.5.2](https://github.com/koesterlab/alignoth/compare/v0.5.1...v0.5.2) (2022-10-26)


### Bug Fixes

* Fix highlighting region for html and json output ([5ce5d17](https://github.com/koesterlab/alignoth/commit/5ce5d172319c523096d3b7baca672f7ca220bf30))

## [0.5.1](https://github.com/koesterlab/alignoth/compare/v0.5.0...v0.5.1) (2022-10-24)


### Bug Fixes

* fix interval position ([#35](https://github.com/koesterlab/alignoth/issues/35)) ([a2ddcfa](https://github.com/koesterlab/alignoth/commit/a2ddcfacbf779d6521596d97107bace77ead13d9))

## [0.5.0](https://github.com/koesterlab/alignoth/compare/v0.4.0...v0.5.0) (2022-09-26)


### Features

* Increase mapq border size ([#27](https://github.com/koesterlab/alignoth/issues/27)) ([f409509](https://github.com/koesterlab/alignoth/commit/f4095091932c6d292e24be73c3f5faa1d0d99418))


### Bug Fixes

* Reverse map color scale ([#25](https://github.com/koesterlab/alignoth/issues/25)) ([d6bac11](https://github.com/koesterlab/alignoth/commit/d6bac1121da17c51290b46f2052ff69de51e914a))

## [0.4.0](https://github.com/koesterlab/alignoth/compare/v0.3.0...v0.4.0) (2022-08-24)


### Features

* Add option to output resulting plot in an html file ([#14](https://github.com/koesterlab/alignoth/issues/14)) ([e36ebc4](https://github.com/koesterlab/alignoth/commit/e36ebc45159262dc0956dd7c202d70747bc2b0f9))
* Resize markers of read plots ([#16](https://github.com/koesterlab/alignoth/issues/16)) ([748d180](https://github.com/koesterlab/alignoth/commit/748d1805c41e5dac164db59feedacf8a50e4035c))

## [0.3.0](https://github.com/koesterlab/alignoth/compare/v0.2.2...v0.3.0) (2022-07-25)


### Features

* Support multiple output formats ([#11](https://github.com/koesterlab/alignoth/issues/11)) ([53bfe08](https://github.com/koesterlab/alignoth/commit/53bfe08f06d1a6e4b81267a23a5da93d76ea4425))

## [0.2.2](https://github.com/koesterlab/alignoth/compare/v0.2.1...v0.2.2) (2022-06-27)


### Bug Fixes

* Fix output of highlight data ([#8](https://github.com/koesterlab/alignoth/issues/8)) ([cd8c42d](https://github.com/koesterlab/alignoth/commit/cd8c42d094fd24ef0fc38b8eb2c6b02a75a28209))

## [0.2.1](https://github.com/koesterlab/alignoth/compare/v0.2.0...v0.2.1) (2022-06-23)


### Bug Fixes

* Fix index out of bounce panic for reads with deletions ([#6](https://github.com/koesterlab/alignoth/issues/6)) ([f412be7](https://github.com/koesterlab/alignoth/commit/f412be7866f9676ed16e5092378187b4b103000e))

## [0.2.0](https://github.com/koesterlab/alignoth/compare/v0.1.0...v0.2.0) (2022-06-22)


### Features

* Add FromStr Implementation for PlotCigar ([ad5ede0](https://github.com/koesterlab/alignoth/commit/ad5ede0c5335ab780a121e0f6b8a04e0243697f3))
* Change outputs to be file paths instead of directories ([9773472](https://github.com/koesterlab/alignoth/commit/97734729fdc8a748fdf30656e795a4836ba76536))


### Bug Fixes

* Fix read placement for positions smaller than 5 ([ad5ede0](https://github.com/koesterlab/alignoth/commit/ad5ede0c5335ab780a121e0f6b8a04e0243697f3))

## 0.1.0 (2022-06-22)


### Miscellaneous Chores

* release 0.1.0 ([d2a9719](https://github.com/koesterlab/alignoth/commit/d2a97197c59aa9465e58025e1a21218a22658896))
