// @ts-check

import { WebpEncoder } from '../index.js'
import { GIFEncoder } from '@gomander/napi-gif-encoder'
import { rgb, circleAndSquare } from './animations.js'

async function webpRgb() {
  const buffers = rgb()
  const encoder = new WebpEncoder(100, 100)

  encoder.setFrameRate(3)

  console.time('Encoding rgb as WebP')
  for (const buffer of buffers) {
    encoder.addFrame(buffer)
  }

  await encoder.writeToFile('rgb.webp')
  console.timeEnd('Encoding rgb as WebP')
}

async function gifRgb() {
  const buffers = rgb()
  const encoder = new GIFEncoder(100, 100, 'rgb.gif')

  encoder.setFrameRate(3)

  console.time('Encoding rgb as GIF')
  for (const buffer of buffers) {
    encoder.addFrame(buffer)
  }

  await encoder.finish()
  console.timeEnd('Encoding rgb as GIF')
}

async function webpCircleAndSquare() {
  const buffers = circleAndSquare()
  const encoder = new WebpEncoder(100, 100)

  encoder.setFrameRate(20)

  console.time('Encoding circle and square as WebP')
  for (const buffer of buffers) {
    encoder.addFrame(buffer)
  }

  await encoder.writeToFile('circle-and-square.webp')
  console.timeEnd('Encoding circle and square as WebP')
}

async function gifCircleAndSquare() {
  const buffers = circleAndSquare()
  const encoder = new GIFEncoder(100, 100, 'circle-and-square.gif')

  encoder.setFrameRate(20)

  console.time('Encoding circle and square as GIF')
  for (const buffer of buffers) {
    encoder.addFrame(buffer)
  }

  await encoder.finish()
  console.timeEnd('Encoding circle and square as GIF')
}

async function main() {
  await webpRgb()
  await gifRgb()

  await webpCircleAndSquare()
  await gifCircleAndSquare()
}

main()
