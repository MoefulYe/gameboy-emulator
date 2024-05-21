import { type InjectionKey, inject } from 'vue'
import { Emulator } from './emulator'
import { Button as EmulatorButton } from 'emulator/pkg'

export const emuKey = Symbol() as InjectionKey<Emulator>

export const getEmulator = async (): Promise<Emulator> => {
  const { emulator } = await import('./emulator')
  return emulator
}

export const useEmulator = () => inject(emuKey)!

export { Emulator, EmulatorButton }
