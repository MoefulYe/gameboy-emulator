import { createEventEmitter, type EventEmitter } from '@/event'

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
