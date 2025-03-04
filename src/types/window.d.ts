export declare global {
  interface Window {
    emulatorLogCallback(
      logs: {
        level: 0 | 1 | 2 | 3 | 4
        msg: string
      }[]
    ): void
    emulatorSerialCallback(bytes: Uint8Array): void
    emulatorAudioCallback(data: Float32Array): void
  }
}
