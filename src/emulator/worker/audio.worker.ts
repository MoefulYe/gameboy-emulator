import { AudioReceiver } from '../output/audio'

class EmuAudioProcessor extends AudioWorkletProcessor {
  queue: [Float32Array, Float32Array][] = []
  constructor() {
    super()
    this.port.onmessage = (e) => {
      const { port } = e.data
      new AudioReceiver(port).recv((left, right) => {
        this.queue.push([left, right])
      })
    }
  }
  process(
    inputs: Float32Array[][],
    outputs: Float32Array[][],
    parameters: Record<string, Float32Array>
  ): boolean {
    const output = outputs[0]
    if (this.queue.length > 0) {
      const [left, right] = this.queue.shift()!
      output[0] = left
      output[1] = right
    } else {
      output[0].fill(0)
      output[1].fill(0)
    }
    return true
  }
}

registerProcessor('audio-processor', EmuAudioProcessor)
