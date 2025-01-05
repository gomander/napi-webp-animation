# `napi-webp-animation`

![https://github.com/gomander/napi-webp-animation/actions](https://github.com/gomander/napi-webp-animation/workflows/CI/badge.svg)

> A WebP animation package for NodeJS using Rust

## Install

```sh
npm install napi-webp-animation
```

## Support matrix

|                  | node18 | node20 | node22 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS aarch64    | ✓      | ✓      | ✓      |


## Developing

- Install latest [Rust](https://rustup.rs/). If on Windows, use WSL for an easier time.
- Install `NodeJS@18+`. LTS versions suggested.
- Install `pnpm` and dependencies with `pnpm i`.

You can then compile the rust code with `pnpm build`.

After running `pnpm build`, you will see a
`napi-gif-encoder.<PLATFORM>.node` file in the project root.
This is the native addon built from [lib.rs](./src/lib.rs).
