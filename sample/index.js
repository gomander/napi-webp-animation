// @ts-check

import { readFile } from 'node:fs/promises'
import { WebpEncoder, decodeWebp } from '../index.js'
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

/**
 * @param {string} inputPath
 * @param {string} outputPath
 * @returns {Promise<void>}
 */
async function reverseWebp(inputPath, outputPath) {
  const buffer = await readFile(inputPath)
  const decodedWebp = decodeWebp(buffer)
  const encoder = new WebpEncoder(decodedWebp.width, decodedWebp.height)
  encoder.setFrameRate(20)
  for (let i = decodedWebp.frames.length - 1; i >= 0; i--) {
    const frame = decodedWebp.frames[i]
    encoder.addFrame(frame.data)
  }
  await encoder.writeToFile(outputPath)
}

async function main() {
  await webpRgb()
  await gifRgb()

  await webpCircleAndSquare()
  await gifCircleAndSquare()

  await reverseWebp('circle-and-square.webp', 'circle-and-square-reversed.webp')
}

main()
