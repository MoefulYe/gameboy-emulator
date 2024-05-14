import { createEventEmitter, type EventEmitter } from '@/utils/event'

export const enum LogLevel {
  Off = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Debug = 4,
  Trace = 5
}

export type EmulatorEvent = {
  log: [level: LogLevel, msg: string]
  serial: [byte: number]
}

export type EmulatorEventType = keyof EmulatorEvent

export const createEmulatorEventEmitter = (): EventEmitter<EmulatorEvent> => {
  const emitter = createEventEmitter<EmulatorEvent>()
  window.emulatorLogCallback = (level: LogLevel, msg: string) => emitter.emit('log', level, msg)
  window.emulatorSerialCallback = (byte: number) => emitter.emit('serial', byte)
  return emitter
}
