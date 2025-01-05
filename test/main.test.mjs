// @ts-check

import assert from 'node:assert'
import fs from 'node:fs/promises'
import { dirname } from 'node:path'
import test from 'node:test'
import { fileURLToPath } from 'node:url'
import { WebpEncoder } from '../index.js'
import { Canvas, loadImage } from '@napi-rs/canvas'

/** @returns {Promise<Buffer[]>} */
async function getFrames() {
  /** @type {Buffer[]} */
  const frames = []
  const ctx = new Canvas(480, 270).getContext('2d')
  for (let i = 1; i <= 17; i++) {
    const path = `${dirname(fileURLToPath(import.meta.url))}/fixtures/frames/${i}.png`
    const file = await fs.readFile(path)
    const img = await loadImage(file)
    ctx.drawImage(img, 0, 0)
    const { data } = ctx.getImageData(0, 0, 480, 270)
    frames.push(Buffer.from(data))
  }
  return frames
}

test('encodes webp', async () => {
  const frames = await getFrames()
  const encoder = new WebpEncoder(480, 270)

  encoder.setFrameRate(24)

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
