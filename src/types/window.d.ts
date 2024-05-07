export declare global {
  type LogLevel = 'off' | 'trace' | 'debug' | 'info' | 'warn' | 'error'

  interface Window {
    emulatorLogCallback(level: LogLevel, msg: string)
    emulatorSerialCallback(byte: number)
  }
}
