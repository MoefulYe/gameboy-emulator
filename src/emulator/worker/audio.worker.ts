import { AudioReceiver } from '../output/audio'

class RingBuffer {
  private size: number
  private buffer: Float32Array
  private writePtr: number
  private readPtr: number
  private available: number
  constructor(size: number) {
    this.size = size
    this.buffer = new Float32Array(size)
    this.writePtr = 0
    this.readPtr = 0
    this.available = 0 // 可读的样本数
  }

  write(data: Float32Array) {
    if (data.length === 0) return
    const writeAmount = Math.min(data.length, this.size - this.available)
    if (writeAmount <= 0) return 0
    const writePtr = this.writePtr
    const end = writePtr + writeAmount
    if (end <= this.size) {
      this.buffer.set(data.subarray(0, writeAmount), writePtr)
      this.writePtr = end % this.size
    } else {
      const firstPart = this.size - writePtr
      this.buffer.set(data.subarray(0, firstPart), writePtr)
      const secondPart = writeAmount - firstPart
      this.buffer.set(data.subarray(firstPart, firstPart + secondPart), 0)
      this.writePtr = secondPart
    }
    this.available += writeAmount
    return writeAmount
  }

  read(output: Float32Array) {
    const readAmount = Math.min(output.length, this.available)
    if (readAmount <= 0) {
      output.fill(0)
      return 0
    }
    const readPtr = this.readPtr
    const end = readPtr + readAmount
    if (end <= this.size) {
      output.set(this.buffer.subarray(readPtr, end))
      this.readPtr = end % this.size
    } else {
      const firstPart = this.size - readPtr
      output.set(this.buffer.subarray(readPtr, this.size), 0)
      const secondPart = readAmount - firstPart
      output.set(this.buffer.subarray(0, secondPart), firstPart)
      this.readPtr = secondPart
    }
    this.available -= readAmount
    return readAmount
  }

  clear() {
    this.writePtr = 0
    this.readPtr = 0
    this.available = 0
    this.buffer.fill(0)
  }
}

class EmuAudioProcessor extends AudioWorkletProcessor {
  buffer_left = new RingBuffer(4096)
  buffer_right = new RingBuffer(4096)
  lastUpdate = 0
  constructor() {
    super()
    this.port.onmessage = (e) => {
      const { port } = e.data
      new AudioReceiver(port).recv((left, right) => {
        this.buffer_left.write(left)
        this.buffer_right.write(right)
      })
    }
  }

  process(_: Float32Array[][], outputs: Float32Array[][]): boolean {
    const [left, right] = outputs[0]
    this.buffer_left.read(left)
    this.buffer_right.read(right)
    return true
  }
}

registerProcessor('audio-processor', EmuAudioProcessor)
