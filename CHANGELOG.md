# Changelog

## [1.4.2](https://github.com/alignoth/alignoth/compare/v1.4.1...v1.4.2) (2026-01-14)


### Bug Fixes

* Allow .vcf files in wizard mode ([#414](https://github.com/alignoth/alignoth/issues/414)) ([e0ea1fb](https://github.com/alignoth/alignoth/commit/e0ea1fbd94d41e87dec8400237b89cc400c2cccb))
* Clamp region to reference bounds generated via around in wizard ([eb3e064](https://github.com/alignoth/alignoth/commit/eb3e064ec03915c0ffb818dc506ed79154c27069))
* Clamp region to reference bounds generated via around in wizard mode ([#415](https://github.com/alignoth/alignoth/issues/415)) ([eb3e064](https://github.com/alignoth/alignoth/commit/eb3e064ec03915c0ffb818dc506ed79154c27069))

## [1.4.1](https://github.com/alignoth/alignoth/compare/v1.4.0...v1.4.1) (2025-11-14)


### Performance Improvements

* Simplify Vega transforms for coverage base counts ([#407](https://github.com/alignoth/alignoth/issues/407)) ([7d094d5](https://github.com/alignoth/alignoth/commit/7d094d5bfea9317e7c29a493c78175d3096652ce))

## [1.4.0](https://github.com/alignoth/alignoth/compare/v1.3.0...v1.4.0) (2025-11-13)


### Features

* Add mismatch display threshold option ([#405](https://github.com/alignoth/alignoth/issues/405)) ([0c77b11](https://github.com/alignoth/alignoth/commit/0c77b110218a026192591fd11b9fee825750af28))


### Bug Fixes

* Strip multiple file extensions from BAM name ([cab3062](https://github.com/alignoth/alignoth/commit/cab3062066115ab4700aa2260118cc0dccd26252))
* Update plot color palettes for color blindness and increase opacity ([#406](https://github.com/alignoth/alignoth/issues/406)) ([ce1bac7](https://github.com/alignoth/alignoth/commit/ce1bac744195088964321513d145ade0358dbedf))

## [1.3.0](https://github.com/alignoth/alignoth/compare/v1.2.1...v1.3.0) (2025-10-30)


### Features

* Add --around-vcf-record option to plot region around VCF record by ([e82e8c3](https://github.com/alignoth/alignoth/commit/e82e8c3a07162271f2767694d46a7fd050b75412))
* Add --around-vcf-record option to plot region around VCF record by index ([#401](https://github.com/alignoth/alignoth/issues/401)) ([e82e8c3](https://github.com/alignoth/alignoth/commit/e82e8c3a07162271f2767694d46a7fd050b75412))

## [1.2.1](https://github.com/alignoth/alignoth/compare/v1.2.0...v1.2.1) (2025-10-30)


### Bug Fixes

* Remove potential coverage plot offset at inital plot rendering ([#399](https://github.com/alignoth/alignoth/issues/399)) ([10e6dad](https://github.com/alignoth/alignoth/commit/10e6dad238b06c971bb4c0c3ae43075a179af251))

## [1.2.0](https://github.com/alignoth/alignoth/compare/v1.1.6...v1.2.0) (2025-10-30)


### Features

* Add tooltip to highlight mark showing name field ([#398](https://github.com/alignoth/alignoth/issues/398)) ([c3242c3](https://github.com/alignoth/alignoth/commit/c3242c3ea7b526e8f46691fdeed9d03788816e2f))


### Bug Fixes

* Clarify highlight input prompt with interval and position examples ([7a6d5fd](https://github.com/alignoth/alignoth/commit/7a6d5fd69bcbc00c4941e3f0a4c05db4ffeaa050))

## [1.1.6](https://github.com/alignoth/alignoth/compare/v1.1.5...v1.1.6) (2025-10-29)


### Bug Fixes

* Check for both .csi and .tbi VCF index files ([#395](https://github.com/alignoth/alignoth/issues/395)) ([dd5059f](https://github.com/alignoth/alignoth/commit/dd5059f89f5963ea9bd30c25d64d1ce67cd1689f))

## [1.1.5](https://github.com/alignoth/alignoth/compare/v1.1.4...v1.1.5) (2025-10-28)


### Bug Fixes

* Write highlight output if VCF or BED is provided ([#393](https://github.com/alignoth/alignoth/issues/393)) ([0ee280f](https://github.com/alignoth/alignoth/commit/0ee280f2b9ff83588a6b62d81af2b203a5e4b450))

## [1.1.4](https://github.com/alignoth/alignoth/compare/v1.1.3...v1.1.4) (2025-10-28)


### Bug Fixes

* Make sure reference start is always interpreted as integer ([#391](https://github.com/alignoth/alignoth/issues/391)) ([291dbb9](https://github.com/alignoth/alignoth/commit/291dbb984006568ed1a8290ad8c1b60540f88ae3))

## [1.1.3](https://github.com/alignoth/alignoth/compare/v1.1.2...v1.1.3) (2025-10-27)


### Bug Fixes

* Create output directory if it does not exist ([#389](https://github.com/alignoth/alignoth/issues/389)) ([43a3c12](https://github.com/alignoth/alignoth/commit/43a3c12583ff4e821de9348427ef33c7f7b9c914))

## [1.1.2](https://github.com/alignoth/alignoth/compare/v1.1.1...v1.1.2) (2025-10-24)


### Bug Fixes

* Ensure correct typing of start field ([#386](https://github.com/alignoth/alignoth/issues/386)) ([bb05b6a](https://github.com/alignoth/alignoth/commit/bb05b6a0605636bc9db77b63bf4ac61f8fbb61cf))

## [1.1.1](https://github.com/alignoth/alignoth/compare/v1.1.0...v1.1.1) (2025-10-21)


### Bug Fixes

* Fixate stack order in coverage plot ([#384](https://github.com/alignoth/alignoth/issues/384)) ([ff45355](https://github.com/alignoth/alignoth/commit/ff45355b6d33cbf23bc051c00159c3d4936aa976))

## [1.1.0](https://github.com/alignoth/alignoth/compare/v1.0.2...v1.1.0) (2025-10-21)


### Features

* Add option to disable embedded JS in generated HTML ([#382](https://github.com/alignoth/alignoth/issues/382)) ([36ba851](https://github.com/alignoth/alignoth/commit/36ba851f0f13afc74e8755b2d595ce16c707f9dc))


### Performance Improvements

* Use indexed reader for VCF highlighting parameters ([#381](https://github.com/alignoth/alignoth/issues/381)) ([4a6bf64](https://github.com/alignoth/alignoth/commit/4a6bf645fded28faf7e497acfbb146516674411c))

## [1.0.2](https://github.com/alignoth/alignoth/compare/v1.0.1...v1.0.2) (2025-10-20)


### Performance Improvements

* Refactor coverage encoding for sparse base tracks and diffs ([#379](https://github.com/alignoth/alignoth/issues/379)) ([3a42840](https://github.com/alignoth/alignoth/commit/3a428404bf8d8ab45a6d5dc331d76c6f684d992b))

## [1.0.1](https://github.com/alignoth/alignoth/compare/v1.0.0...v1.0.1) (2025-10-16)


### Bug Fixes

* Rename test file due to special character preventing crate release ([1998228](https://github.com/alignoth/alignoth/commit/19982284943ca8d2c556bbf3e8a3b45b10bd2e9b))

## [1.0.0](https://github.com/alignoth/alignoth/compare/v0.16.5...v1.0.0) (2025-10-16)


### ⚠ BREAKING CHANGES

* Update highlight mechanism with multiple highlighting region and VCF and BED options ([#372](https://github.com/alignoth/alignoth/issues/372))

### Features

* Add per-base coverage breakdown to Coverage struct and plot ([#368](https://github.com/alignoth/alignoth/issues/368)) ([5373868](https://github.com/alignoth/alignoth/commit/5373868b177be0586db13ac2ce386af45ba22e38))
* Update highlight mechanism with multiple highlighting region and VCF and BED options ([#372](https://github.com/alignoth/alignoth/issues/372)) ([9adbf16](https://github.com/alignoth/alignoth/commit/9adbf16693438f4dafa35d5faa04d5fde226e216))


### Bug Fixes

* Change area chart interpolation from monotone to step ([cabc396](https://github.com/alignoth/alignoth/commit/cabc396eb859186e011f3685402f68fc69772809))
* Improve read placement by fixing read end-position calculation ([#371](https://github.com/alignoth/alignoth/issues/371)) ([5e0b8ca](https://github.com/alignoth/alignoth/commit/5e0b8ca19451a517e5b8297e6cecdc437e13ddf6))
* Move subtitle to y axis title ([#370](https://github.com/alignoth/alignoth/issues/370)) ([eb029df](https://github.com/alignoth/alignoth/commit/eb029dfab009a6e6ce823af8a751bbeefdadd11f))

## [0.16.5](https://github.com/alignoth/alignoth/compare/v0.16.4...v0.16.5) (2025-10-13)


### Bug Fixes

* Package js libraries into alignoth ([#365](https://github.com/alignoth/alignoth/issues/365)) ([0a890a5](https://github.com/alignoth/alignoth/commit/0a890a5bc3b86a2201f23bf490e3eab97f4c8cb6))

## [0.16.4](https://github.com/alignoth/alignoth/compare/v0.16.3...v0.16.4) (2025-10-10)


### Bug Fixes

* Handle CIGAR operations when calculating coverage ([#363](https://github.com/alignoth/alignoth/issues/363)) ([31d0775](https://github.com/alignoth/alignoth/commit/31d07751bb7d65de080c8e408927e65144062986))

## [0.16.3](https://github.com/alignoth/alignoth/compare/v0.16.2...v0.16.3) (2025-08-25)


### Performance Improvements

* Compress specs using lz-string for HTML output ([#350](https://github.com/alignoth/alignoth/issues/350)) ([65fc73b](https://github.com/alignoth/alignoth/commit/65fc73bd99028f32be867f57560ef9e6b370ff28))

## [0.16.2](https://github.com/alignoth/alignoth/compare/v0.16.1...v0.16.2) (2025-07-08)


### Bug Fixes

* Allow SAM and CRAM files for wizard usage ([#340](https://github.com/alignoth/alignoth/issues/340)) ([70a65e2](https://github.com/alignoth/alignoth/commit/70a65e2547501e222cde75a1bd67fcf7abfa3b39))

## [0.16.1](https://github.com/alignoth/alignoth/compare/v0.16.0...v0.16.1) (2025-05-28)


### Bug Fixes

* Resolve MAPQ border issue in VL specs ([a4a622e](https://github.com/alignoth/alignoth/commit/a4a622e5defa78223a4ab3af4059c151a4775f91))

## [0.16.0](https://github.com/alignoth/alignoth/compare/v0.15.0...v0.16.0) (2025-05-28)


### Features

* Add coverage plot ([#332](https://github.com/alignoth/alignoth/issues/332)) ([0fb3f14](https://github.com/alignoth/alignoth/commit/0fb3f14ae6c484085364dc27f4e0dbc55712eda8))
* Improved visualization for read pairs ([#330](https://github.com/alignoth/alignoth/issues/330)) ([02aaef3](https://github.com/alignoth/alignoth/commit/02aaef3ea4aaa80ef62139059d846d7071b94367))

## [0.15.0](https://github.com/alignoth/alignoth/compare/v0.14.3...v0.15.0) (2025-05-26)


### Features

* Add new wizard mode for interactive plot generation ([#327](https://github.com/alignoth/alignoth/issues/327)) ([25d099e](https://github.com/alignoth/alignoth/commit/25d099edac84a85efc72540006ef96915d95e7e7))

## [0.14.3](https://github.com/alignoth/alignoth/compare/v0.14.2...v0.14.3) (2025-05-15)


### Performance Improvements

* Update release profile ([ef671e3](https://github.com/alignoth/alignoth/commit/ef671e347d700c6419c3f105d766f4d637ca2806))

## [0.14.2](https://github.com/alignoth/alignoth/compare/v0.14.1...v0.14.2) (2025-05-14)


### Performance Improvements

* Improve storage footprint ([9c3dd3f](https://github.com/alignoth/alignoth/commit/9c3dd3fcffa3116793c14115c747786ccc62650b))

## [0.14.1](https://github.com/alignoth/alignoth/compare/v0.14.0...v0.14.1) (2025-05-08)


### Bug Fixes

* Fix library fetching by downgrading reqwest to 0.11 ([dffcc01](https://github.com/alignoth/alignoth/commit/dffcc01dc30f1054ec0b673f7bc879038b410826))
* Fix library fetching by downgrading reqwest to 0.11 ([3ee2348](https://github.com/alignoth/alignoth/commit/3ee2348899ea1e28590d4ced4c1b777052c776d3))

## [0.14.0](https://github.com/alignoth/alignoth/compare/v0.13.0...v0.14.0) (2025-05-06)


### Features

* Add filtering functionality and improve layout for plot controls ([#318](https://github.com/alignoth/alignoth/issues/318)) ([e246dc8](https://github.com/alignoth/alignoth/commit/e246dc8c2dc9677254175938b0eee3437ef0a69b))
* Add interactive read info table and styling to plot page ([#316](https://github.com/alignoth/alignoth/issues/316)) ([493332b](https://github.com/alignoth/alignoth/commit/493332b7955e90178dfe8633f47b029e7a6e7be1))
* Add raw_cigar field to tooltip ([#319](https://github.com/alignoth/alignoth/issues/319)) ([05b6282](https://github.com/alignoth/alignoth/commit/05b6282e61540574e316c4c6757030d2a7fbb5c1))
* Enhanced multi-read selection with shift-click functionality ([#320](https://github.com/alignoth/alignoth/issues/320)) ([b9d6453](https://github.com/alignoth/alignoth/commit/b9d6453af1264c3579d65ee78bbac097ab64a750))


### Bug Fixes

* Adjust control panel margins in html view ([85eacba](https://github.com/alignoth/alignoth/commit/85eacba530776bb58715351562b43d8290a286f8))
* Keep controls fixed at top ([#321](https://github.com/alignoth/alignoth/issues/321)) ([4e8c1a5](https://github.com/alignoth/alignoth/commit/4e8c1a584cf68f29fa779d4a3cba599379d042ba))

## [0.13.0](https://github.com/alignoth/alignoth/compare/v0.12.1...v0.13.0) (2024-09-18)


### Features

* Add manual for plot usage into HTML output ([#269](https://github.com/alignoth/alignoth/issues/269)) ([47d0f1d](https://github.com/alignoth/alignoth/commit/47d0f1df66502ed785dd7b72f48546fa644af4f0))

## [0.12.1](https://github.com/alignoth/alignoth/compare/v0.12.0...v0.12.1) (2024-07-01)


### Bug Fixes

* Fix potentially unplaced read error ([#244](https://github.com/alignoth/alignoth/issues/244)) ([49dc637](https://github.com/alignoth/alignoth/commit/49dc637c446146a1327c4c78c04707ab8bfc77da))
* Fix read placement ([49dc637](https://github.com/alignoth/alignoth/commit/49dc637c446146a1327c4c78c04707ab8bfc77da))

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
