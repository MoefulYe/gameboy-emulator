export declare global {
  interface Window {
    emulatorLogCallback(level: 0 | 1 | 2 | 3 | 4 | 5, msg: string): void
    emulatorSerialCallback(byte: number): void
  }
}
