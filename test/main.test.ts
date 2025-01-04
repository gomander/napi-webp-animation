import fs from 'node:fs/promises'
import { it, expect } from 'vitest'
import { WebpEncoder } from '..'
import { Canvas, loadImage } from '@napi-rs/canvas'

async function getFrames(): Promise<Buffer[]> {
  const frames: Buffer[] = []
  const ctx = new Canvas(480, 270).getContext('2d')
  for (let i = 1; i <= 17; i++) {
    const file = await fs.readFile(`${import.meta.dirname}/fixtures/frames/${i}.png`)
    const img = await loadImage(file)
    ctx.drawImage(img, 0, 0)
    frames.push(Buffer.from(ctx.getImageData(0, 0, 480, 270).data))
  }
  return frames
}

it('encodes webp', async () => {
  const frames = await getFrames()
  const encoder = new WebpEncoder(480, 270, { lossless: false, quality: 75 })

  encoder.setFrameRate(24)

  for (const frame of frames) {
    encoder.addFrame(frame)
  }
  const data = encoder.writeToFileSync('test/output/test.webp')
  expect(data.length).toBe(178436)
})
