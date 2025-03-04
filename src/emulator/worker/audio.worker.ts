import { AudioReceiver } from '../output/audio'

class EmuAudioProcessor extends AudioWorkletProcessor {
  segments: Float32Array[] = []
  constructor() {
    super()
    this.port.onmessage = (e) => {
      const { port } = e.data
      const segment = new Float32Array(128)
      let offset = 0
      new AudioReceiver(port).recv((data) => {
        segment.set(data.subarray(0, 128 - offset), offset)
        this.segments.push(new Float32Array(segment))
        data = data.subarray(128 - offset)
        while (data.length > 128) {
          const segment = data.subarray(0, 128)
          this.segments.push(segment)
          data = data.subarray(128)
        }
        offset = data.length
        segment.set(data)
      })
    }
  }

  process(_: Float32Array[][], outputs: Float32Array[][]): boolean {
    const segment = this.segments.shift()
    const output = outputs[0][0]
    if (segment != undefined) {
      output.set(segment)
    }
    return true
  }
}

registerProcessor('audio-processor', EmuAudioProcessor)
