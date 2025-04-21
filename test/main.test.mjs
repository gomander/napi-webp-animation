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

test('encodes webp', async () => {
  const frames = await getFrames()
  const encoder = new WebpEncoder(WIDTH, HEIGHT)

  encoder.setFrameRate(FRAME_RATE)

  for (const frame of frames) {
    encoder.addFrame(frame)
  }
  const data = encoder.writeToFileSync(
    'test/output/test.webp',
    { lossless: false, quality: 75 }
  )
  assert.strictEqual(data.length, 178436)
})

/**
 * Test values:
 * lossy 50: 133280
 * lossy 75:  178436
 * lossless 0: 1229416
 * lossless 50: 1173410
 * lossless 100: 1168740
 */

test('decodes webp', async () => {
  const buffer = await fs.readFile('test/output/test.webp')
  const decodedWebp = decodeWebp(buffer)
  assert.strictEqual(decodedWebp.width, WIDTH)
  assert.strictEqual(decodedWebp.height, HEIGHT)
  assert.strictEqual(decodedWebp.frames.length, FRAME_COUNT)
  const expectedFrameSize = WIDTH * HEIGHT * 4
  const duration = Math.floor(1000 / FRAME_RATE)
  let cumulativeTimestamp = 0
  for (const frame of decodedWebp.frames) {
    cumulativeTimestamp += duration
    assert.strictEqual(frame.data.length, expectedFrameSize)
    assert.strictEqual(frame.timestamp, cumulativeTimestamp);
  }
})
