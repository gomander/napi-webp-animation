// @ts-check

import { WebpEncoder } from '../index.js'
import { Canvas } from '@napi-rs/canvas'
import { GIFEncoder } from '@gomander/napi-gif-encoder'

function encodeWebp() {
  const canvas = new Canvas(100, 100)
  const ctx = canvas.getContext('2d')

  const encoder = new WebpEncoder(100, 100)

  encoder.setFrameRate(3)

  console.time('Encoding WebP')
  ctx.fillStyle = 'red'
  ctx.fillRect(0, 0, 100, 100)

  encoder.addFrame(canvas.data())

  ctx.fillStyle = 'green'
  ctx.fillRect(0, 0, 100, 100)

  encoder.addFrame(canvas.data())

  ctx.fillStyle = 'blue'
  ctx.fillRect(0, 0, 100, 100)

  encoder.addFrame(canvas.data())

  encoder.writeToFile('output.webp')
  console.timeEnd('Encoding WebP')
}

async function encodeGif() {
  const canvas = new Canvas(100, 100)
  const ctx = canvas.getContext('2d')

  const gifEncoder = new GIFEncoder(100, 100, 'output.gif')

  gifEncoder.setFrameRate(3)

  console.time('Encoding GIF')
  ctx.fillStyle = 'red'
  ctx.fillRect(0, 0, 100, 100)

  gifEncoder.addFrame(structuredClone(canvas.data()))

  ctx.fillStyle = 'green'
  ctx.fillRect(0, 0, 100, 100)

  gifEncoder.addFrame(structuredClone(canvas.data()))

  ctx.fillStyle = 'blue'
  ctx.fillRect(0, 0, 100, 100)

  gifEncoder.addFrame(structuredClone(canvas.data()))

  await gifEncoder.finish()
  console.timeEnd('Encoding GIF')
}

encodeWebp()
encodeGif()
