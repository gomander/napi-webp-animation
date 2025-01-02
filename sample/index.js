// @ts-check

import { WebpEncoder } from '../index.js'
import { Canvas } from '@napi-rs/canvas'

const encoder = new WebpEncoder(100, 100, 'output.webp', { loopCount: 1, lossless: true, quality: 100, method: 6 })

const canvas = new Canvas(100, 100)
const ctx = canvas.getContext('2d')
ctx.fillStyle = 'red'
ctx.fillRect(0, 0, 100, 100)

encoder.addFrame(canvas.data(), 1000)

ctx.fillStyle = 'green'
ctx.fillRect(0, 0, 100, 100)

encoder.addFrame(canvas.data(), 1000)

ctx.fillStyle = 'blue'
ctx.fillRect(0, 0, 100, 100)

encoder.addFrame(canvas.data(), 1000)

encoder.finish(2000)
