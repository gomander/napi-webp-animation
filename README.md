# `napi-webp-animation`

![https://github.com/gomander/napi-webp-animation/actions](https://github.com/gomander/napi-webp-animation/workflows/CI/badge.svg)

> A WebP animation package for NodeJS using Rust

## Install

```sh
npm install napi-webp-animation
```

## Support matrix

### Operating Systems

| Linux x64/aarch64 | Windows x64 |
| ----------------- | ----------- |
| ✓                 | ✓           |

### NodeJS

Theoretically, any version of Node that supports N-API should work. The CI is
validated against LTS versions of Node:

| Node 18 | Node 20 | Node 22 |
| ------- | ------- | ------- |
| ✓       | ✓       | ✓       |

### Building

If you are using this as a dependency, since we use N-API, you don't
need to build anything! However, if you want to tinker with this code
or submit a PR, read below.

## Developing

- Install latest `Rust`. Suggest using [rustup](https://rustup.rs/). If on
  Windows, use WSL for an easier time.
- Install `NodeJS@18+`. LTS versions suggested.
- Install `pnpm` and dependencies with `pnpm i`.

You can then compile the rust code with `pnpm build`.

After running `pnpm build`, you will see a
`napi-gif-encoder.<PLATFORM>.node` file in the project root.
This is the native addon built from [lib.rs](./src/lib.rs).
