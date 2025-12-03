import assert from 'node:assert'
import fs from 'node:fs/promises'
import { dirname } from 'node:path'
import test from 'node:test'
import { fileURLToPath } from 'node:url'
import { WebpEncoder, decodeWebp, type WebpEncoderOptions } from '../index.js'
import { Canvas, loadImage } from '@napi-rs/canvas'

const WIDTH = 480
const HEIGHT = 270
const FRAME_COUNT = 17
const FRAME_RATE = 24

const testValues = {
  lossy: {
    0: [20016],
    50: [133280],
    75: [178436],
    100: [556672]
  },
  lossless: {
    0: [1229416, 1229542],
    50: [1173410, 1173638],
    100: [1168740, 1169022]
  }
} as const

async function getFrames() {
  const frames: Buffer[] = []
  const ctx = new Canvas(WIDTH, HEIGHT).getContext('2d')
  for (let i = 1; i <= FRAME_COUNT; i++) {
    const path =
      `${dirname(fileURLToPath(import.meta.url))}/fixtures/frames/${i}.png`
    const file = await fs.readFile(path)
    const img = await loadImage(file)
    ctx.drawImage(img, 0, 0)
    const { data } = ctx.getImageData(0, 0, WIDTH, HEIGHT)
    frames.push(Buffer.from(data))
  }
  return frames
}

async function createEncoderWithFrames() {
  const frames = await getFrames()
  const encoder = new WebpEncoder(WIDTH, HEIGHT)
  encoder.setFrameRate(FRAME_RATE)
  for (const frame of frames) {
    encoder.addFrame(frame)
  }
  return encoder
}

for (const encodingMode of ['lossy', 'lossless'] as const) {
  for (const [quality, sizes] of Object.entries(testValues[encodingMode])) {
    for (const syncMode of ['async', 'sync'] as const) {
      const path = `test/output/${encodingMode}-${quality}-${syncMode}.webp`
      const options: WebpEncoderOptions = {
        lossless: encodingMode === 'lossless',
        quality: Number(quality)
      }
      const paddedEncodingMode = encodingMode.padStart('lossless'.length)
      const paddedQuality = quality.padStart('100'.length)
      const paddedSyncMode = syncMode.padStart('async'.length)
      test(
        `Encode | ${paddedEncodingMode} | ${paddedQuality} | ${paddedSyncMode}`,
        async () => {
          const encoder = await createEncoderWithFrames()
          const data = syncMode === 'async'
            ? await encoder.writeToFile(path, options)
            : encoder.writeToFileSync(path, options)
          assert.ok(
            sizes.includes(data.byteLength),
            `Encoded size ${data.byteLength} not in expected sizes`
          )
        }
      )
    }
  }
}

test('Decodes WebP', async () => {
  const buffer = await fs.readFile('test/output/lossy-75-sync.webp')
  const decodedWebp = decodeWebp(buffer)
  assert.strictEqual(decodedWebp.width, WIDTH)
  assert.strictEqual(decodedWebp.height, HEIGHT)
  assert.strictEqual(decodedWebp.frames.length, FRAME_COUNT)
  const expectedFrameSize = WIDTH * HEIGHT * 4
  const duration = Math.floor(1000 / FRAME_RATE)
  let cumulativeTimestamp = 0
  for (let i = 0; i < decodedWebp.frames.length; i++) {
    const frame = decodedWebp.frames[i]
    cumulativeTimestamp += duration
    assert.strictEqual(frame.data.length, expectedFrameSize)
    assert.strictEqual(frame.timestamp, cumulativeTimestamp)
    const encoder = new WebpEncoder(WIDTH, HEIGHT)
    encoder.addFrame(frame.data)
    const encodedWebp = await encoder.writeToFile(`test/output/${i + 1}.webp`)
    assert.ok(
      encodedWebp.byteLength > 80000 && encodedWebp.byteLength < 90000,
      `Encoded frame size ${encodedWebp.byteLength} out of expected range`
    )
  }
})
