import type { Emulator } from './emulator'

export const getEmulator = async (): Promise<Emulator> => {
  const { default: emulator } = await import('./emulator')
  return emulator
}
