# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

_Nothing yet_

## [0.3.1] - 2026-08-06

_No significant changes_

## [0.3.0] - 2026-08-04

### Changed

- Improved error messages when invalid values are passed as dimensions, frame rate, or duration ([c28561a](https://github.com/gomander/napi-webp-animation/commit/c28561afae69454650250ba2fe1fa2a8260ed2bd))
- Reduced memory usage of async methods ([0e35c14](https://github.com/gomander/napi-webp-animation/commit/0e35c140b901e4086e86319e23d67e42f8698f39))
- Improved handling of write paths passed to `writeToFile` and `writeToFileSync` ([04a5901](https://github.com/gomander/napi-webp-animation/commit/04a590176ab65d2216b0addd6b92f3aba7917e62))

### Removed

- Dropped Node 20 support ([6d65f88](https://github.com/gomander/napi-webp-animation/commit/6d65f88e98151379d8452548d6e8c297c6ab055f))
- `writeToFile` and `writeToFileSync` no longer return the Buffer on top of writing it to file ([c28561a](https://github.com/gomander/napi-webp-animation/commit/c28561afae69454650250ba2fe1fa2a8260ed2bd))


## [0.2.2] - 2026-01-27

### Fixed

- Tweaked implementation of async methods to hopefully reduce errors ([3ed00f9](https://github.com/gomander/napi-webp-animation/commit/3ed00f9fe3bc31d1fb7255667ae08933d36a7a4a))

## [0.2.1] - 2025-11-08

### Fixed

- Enforced a minimum 1ms duration for each frame to prevent encoding errors ([0c1e13d](https://github.com/gomander/napi-webp-animation/commit/0c1e13d0db50ad46242eaf6e05b3478fc462c588))

## [0.2.0] - 2025-11-01

### Added

- Added `decodeWebp` function and `DecodedWebp` and `WebpFrame` interfaces ([cbec287](https://github.com/gomander/napi-webp-animation/commit/cbec2874d22748a7d4970fef5a302ab7accd4a63))

### Changed

- Renamed `JsWebpEncoderOptions` interface to `WebpEncoderOptions` ([cbec287](https://github.com/gomander/napi-webp-animation/commit/cbec2874d22748a7d4970fef5a302ab7accd4a63))
- Minor memory and performance tweaks ([9835306](https://github.com/gomander/napi-webp-animation/commit/9835306b457610b90cc2bbdbf8ca11e37b5e6d54))

## [0.1.1] - 2025-04-16

_No significant changes_

## [0.1.0] - 2025-01-05

### Changed

- Minor memory and performance tweaks ([487a97b](https://github.com/gomander/napi-webp-animation/commit/487a97baec3dbf1cec986db515fff118e613fdcc))

### Removed

- Removed `method` option, as it did nothing ([050409e](https://github.com/gomander/napi-webp-animation/commit/050409e55511864fee4a762f906184a9df3b91fa))

## [0.0.1] - 2025-01-05

Initial release
