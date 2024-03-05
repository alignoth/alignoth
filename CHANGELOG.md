# Changelog

## [0.12.0](https://github.com/alignoth/alignoth/compare/v0.11.0...v0.12.0) (2024-03-05)


### Features

* Show inserted bases in tooltip ([#212](https://github.com/alignoth/alignoth/issues/212)) ([cacadbe](https://github.com/alignoth/alignoth/commit/cacadbed0af23b4d3bbb37cddaf482878944b903))

## [0.11.0](https://github.com/koesterlab/alignoth/compare/v0.10.0...v0.11.0) (2023-11-15)


### Features

* Show warning in plot when reads are downsampled ([#183](https://github.com/koesterlab/alignoth/issues/183)) ([e7587fc](https://github.com/koesterlab/alignoth/commit/e7587fc84c8e9cd4fab9ca3175a7b125a6e772e2))

## [0.10.0](https://github.com/koesterlab/alignoth/compare/v0.9.0...v0.10.0) (2023-10-04)


### Features

* Allow single selection of a read ([#172](https://github.com/koesterlab/alignoth/issues/172)) ([b365680](https://github.com/koesterlab/alignoth/commit/b3656800a95dd8e6f010065c91998a4204de6d9b))

## [0.9.0](https://github.com/koesterlab/alignoth/compare/v0.8.2...v0.9.0) (2023-09-25)


### Features

* Allow displaying user defined aux tags in read tooltip ([#164](https://github.com/koesterlab/alignoth/issues/164)) ([3e969aa](https://github.com/koesterlab/alignoth/commit/3e969aaec2ed8d31c15710a14395dd136028d683))


### Bug Fixes

* Remove unconnected lines for unpaired reads ([#167](https://github.com/koesterlab/alignoth/issues/167)) ([6e25855](https://github.com/koesterlab/alignoth/commit/6e258553e2ea4e9c133ccbc68d12d7820944bffd))

## [0.8.2](https://github.com/koesterlab/alignoth/compare/v0.8.1...v0.8.2) (2023-05-10)


### Bug Fixes

* Fix non-displayed insertions ([#125](https://github.com/koesterlab/alignoth/issues/125)) ([04c91ce](https://github.com/koesterlab/alignoth/commit/04c91ce83ed926e3a36c2e3aaa9742a9a0d796df))

## [0.8.1](https://github.com/koesterlab/alignoth/compare/v0.8.0...v0.8.1) (2023-04-26)


### Bug Fixes

* Reduce vertical mapq border size ([#119](https://github.com/koesterlab/alignoth/issues/119)) ([554288c](https://github.com/koesterlab/alignoth/commit/554288c9c8d77273eefee33c3b6382fb0fbd2aaf))
* Remove duplicated legend ([#117](https://github.com/koesterlab/alignoth/issues/117)) ([9f2412c](https://github.com/koesterlab/alignoth/commit/9f2412c0a7a62fea1bb2d99dc0c673225c69b8d8))
* Remove end offset of 1 ([#120](https://github.com/koesterlab/alignoth/issues/120)) ([6d63973](https://github.com/koesterlab/alignoth/commit/6d639739500eb6ff4c9af923b0455ffcbe90107a))

## [0.8.0](https://github.com/koesterlab/alignoth/compare/v0.7.3...v0.8.0) (2023-04-14)


### Features

* Show length of deletions via tooltip ([#112](https://github.com/koesterlab/alignoth/issues/112)) ([aad66c2](https://github.com/koesterlab/alignoth/commit/aad66c237ded4174510bf4a86d135ba2730b1989))

## [0.7.3](https://github.com/koesterlab/alignoth/compare/v0.7.2...v0.7.3) (2023-03-31)


### Bug Fixes

* Fix wrong read offset when read starts with softclips ([#104](https://github.com/koesterlab/alignoth/issues/104)) ([16153cf](https://github.com/koesterlab/alignoth/commit/16153cf604332effdc05e22329934009d0dbe9db))

## [0.7.2](https://github.com/koesterlab/alignoth/compare/v0.7.1...v0.7.2) (2023-03-22)


### Bug Fixes

* Clamp around parameter according to target length ([#98](https://github.com/koesterlab/alignoth/issues/98)) ([7db5f07](https://github.com/koesterlab/alignoth/commit/7db5f07011d12318de604de375ea55cf6e7ab340))

## [0.7.1](https://github.com/koesterlab/alignoth/compare/v0.7.0...v0.7.1) (2023-03-03)


### Bug Fixes

* Update htslib ([#86](https://github.com/koesterlab/alignoth/issues/86)) ([022a70c](https://github.com/koesterlab/alignoth/commit/022a70c9f9f2b79a650f71a1b72ff495ac739a1c))

## [0.7.0](https://github.com/koesterlab/alignoth/compare/v0.6.2...v0.7.0) (2023-02-08)


### Features

* Add around parameter ([#72](https://github.com/koesterlab/alignoth/issues/72)) ([5cb48fc](https://github.com/koesterlab/alignoth/commit/5cb48fce922cb08ac3530bc6f0bdfb1a8814c4e3))
* Add option to plot full bam file ([#76](https://github.com/koesterlab/alignoth/issues/76)) ([21fafcf](https://github.com/koesterlab/alignoth/commit/21fafcf03ada66ae93fb17e4e1f67d42ede998de))
* Allow highlighting of single base position ([#77](https://github.com/koesterlab/alignoth/issues/77)) ([8d16897](https://github.com/koesterlab/alignoth/commit/8d168970e3dcb0a97fbf9d97a56457be9156d179))
* Allow omitting reference and bam files if cwd only contains one of each ([#74](https://github.com/koesterlab/alignoth/issues/74)) ([e44ec4d](https://github.com/koesterlab/alignoth/commit/e44ec4d6a3a2c73c77f8f303b32c5fcf43dcd082))


### Bug Fixes

* Fix offset of 1 base for reference and reads ([#81](https://github.com/koesterlab/alignoth/issues/81)) ([48fdaaa](https://github.com/koesterlab/alignoth/commit/48fdaaa27e079f9b31146e68044669a559a479bc))

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
