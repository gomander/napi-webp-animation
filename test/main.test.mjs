// @ts-check

import assert from 'node:assert'
import fs from 'node:fs/promises'
import { dirname } from 'node:path'
import test from 'node:test'
import { fileURLToPath } from 'node:url'
import { WebpEncoder, decodeWebp } from '../index.js'
import { Canvas, loadImage } from '@napi-rs/canvas'

const WIDTH = 480
const HEIGHT = 270
const FRAME_COUNT = 17
const FRAME_RATE = 24

/** @returns {Promise<Buffer[]>} */
async function getFrames() {
  /** @type {Buffer[]} */
  const frames = []
  const ctx = new Canvas(WIDTH, HEIGHT).getContext('2d')
  for (let i = 1; i <= FRAME_COUNT; i++) {
    const path = `${dirname(fileURLToPath(import.meta.url))}/fixtures/frames/${i}.png`
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

test('encode lossy 0 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossy-0-async.webp',
    { lossless: false, quality: 0 }
  )
  assert.strictEqual(data.length, 20016)
})

test('encode lossy 0 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossy-0-sync.webp',
    { lossless: false, quality: 0 }
  )
  assert.strictEqual(data.length, 20016)
})

test('encode lossy 50 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossy-50-async.webp',
    { lossless: false, quality: 50 }
  )
  assert.strictEqual(data.length, 133280)
})

test('encode lossy 50 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossy-50-sync.webp',
    { lossless: false, quality: 50 }
  )
  assert.strictEqual(data.length, 133280)
})

test('encode lossy 75 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossy-75-async.webp',
    { lossless: false, quality: 75 }
  )
  assert.strictEqual(data.length, 178436)
})

test('encode lossy 75 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossy-75-sync.webp',
    { lossless: false, quality: 75 }
  )
  assert.strictEqual(data.length, 178436)
})

test('encode lossy 100 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossy-100-async.webp',
    { lossless: false, quality: 100 }
  )
  assert.strictEqual(data.length, 556672)
})

test('encode lossy 100 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossy-100-sync.webp',
    { lossless: false, quality: 100 }
  )
  assert.strictEqual(data.length, 556672)
})

test('encode lossless 0 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossless-0-async.webp',
    { lossless: true, quality: 0 }
  )
  assert.strictEqual(data.length, 1229542)
})

test('encode lossless 0 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossless-0-sync.webp',
    { lossless: true, quality: 0 }
  )
  assert.strictEqual(data.length, 1229542)
})

test('encode lossless 50 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossless-50-async.webp',
    { lossless: true, quality: 50 }
  )
  assert.strictEqual(data.length, 1173638)
})

test('encode lossless 50 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossless-50-sync.webp',
    { lossless: true, quality: 50 }
  )
  assert.strictEqual(data.length, 1173638)
})

test('encode lossless 100 async', async () => {
  const encoder = await createEncoderWithFrames()

  const data = await encoder.writeToFile(
    'test/output/test-lossless-100-async.webp',
    { lossless: true, quality: 100 }
  )
  assert.strictEqual(data.length, 1169022)
})

test('encode lossless 100 sync', async () => {
  const encoder = await createEncoderWithFrames()

  const data = encoder.writeToFileSync(
    'test/output/test-lossless-100-sync.webp',
    { lossless: true, quality: 100 }
  )
  assert.strictEqual(data.length, 1169022)
})

/**
 * Test values:
 * lossy 0:        20016
 * lossy 50:      133280
 * lossy 75:      178436
 * lossy 100:     556672
 * lossless 0:   1229542
 * lossless 50:  1173638
 * lossless 100: 1169022
 */

test('decodes webp', async () => {
  const buffer = await fs.readFile('test/output/test-lossy-75-sync.webp')
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
    const encodedFrame = await encoder.writeToFile(`test/output/${i + 1}.webp`)
    assert.ok(encodedFrame.byteLength > 80000 && encodedFrame.byteLength < 90000)
  }
})
