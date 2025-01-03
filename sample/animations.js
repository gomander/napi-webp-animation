// @ts-check

import { Canvas } from '@napi-rs/canvas'

/**
 * @returns {Buffer[]}
 */
export function rgb() {
  const canvas = new Canvas(100, 100)
  const ctx = canvas.getContext('2d')
  const buffers = []

  for (const color of ['red', 'green', 'blue']) {
    ctx.fillStyle = color
    ctx.fillRect(0, 0, 100, 100)

    buffers.push(structuredClone(canvas.data()))
  }

  return buffers
}

/**
 * @returns {Buffer[]}
 */
export function circleAndSquare() {
  const canvas = new Canvas(100, 100)
  const ctx = canvas.getContext('2d')
  const buffers = []

  for (let i = 0; i < 100; i++) {
    ctx.fillStyle = 'white'
    ctx.fillRect(0, 0, 100, 100)

    ctx.fillStyle = 'red'
    ctx.fillRect(i * 2 - 50, 25, 50, 50)

    ctx.fillStyle = 'blue'
    ctx.beginPath()
    ctx.arc(50, i * 2 - 50, 25, 0, Math.PI * 2)
    ctx.fill()

    buffers.push(structuredClone(canvas.data()))
  }

  return buffers
}
