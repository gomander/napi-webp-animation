# `napi-webp-animation`

![https://github.com/gomander/napi-webp-animation/actions](https://github.com/gomander/napi-webp-animation/workflows/CI/badge.svg)

> A fully typed WebP animation package for NodeJS using Rust

## ⚠️ This project is a work-in-progress!

This package is a N-API wrapper for the [`webp-animation`](https://crates.io/crates/webp-animation) Rust crate, but does not yet support all its features.  
By virtue of being built with NAPI-RS, this package is fully typed, and compatible with ESM and CJS.

## Install

```sh
npm install napi-webp-animation
```

## Usage

```js
import { WebpEncoder } from 'napi-webp-animation'

// Create an encoder instance with width and height
const encoder = new WebpEncoder(100, 100)

// Set the frame rate of the animated WebP - default 30
encoder.setFrameRate(24)

// Add your frames as buffers
for (const frameBuffer of frameBuffers) {
  encoder.addFrame(frameBuffer)
}

// Encode the animated WebP and write it to file!
const data = await encoder.writeToFile('output.webp')

// Or get the data as a buffer without writing it to file
const data = await encoder.getBuffer()

// Output options can be set on `writeToFile`, `getBuffer`
await encoder.writeToFile('output.webp', {
  lossless: false, // default true
  quality: 75,     // 0 (fast) - 100 (slower) - default 1
  method: 3,       // 0 (fast) - 6 (slower) - default 4
  loopCount: 1     // 0 = infinite, n > 0 = play n times - default 0
})

// Synchronous methods are also available, and also support options
encoder.writeToFileSync('output.webp')
encoder.getBufferSync()

// Clear the encoder's frames, set the dimensions, or set the options
encoder.clearFrames()
encoder.setDimensions(480, 270)
```

## Support matrix

|                  | node18 | node20 | node22 |
| ---------------- | ------ | ------ | ------ |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS aarch64    | ✓      | ✓      | ✓      |
| Windows x64      | ✓      | ✓      | ✓      |

## Contributing

Issues and pull requests are welcome!

Particularly, I would love help setting up the CI to build for more platforms.

## Developing

- If on Windows, recommend using [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
- Install latest [Rust](https://rustup.rs/)
- Install latest LTS [Node](https://nodejs.org/en/download)
- Install pnpm with `npm i -g pnpm`
- Install dependencies with `pnpm i`

Make your changes, and run the tests with `pnpm test`.

You can then compile the rust code with `pnpm build`.

After running `pnpm build`, you will see a
`napi-gif-encoder.<PLATFORM>.node` file in the project root.
This is the native addon built from [lib.rs](./src/lib.rs).
