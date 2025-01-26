import type { Client } from './worker/client'
import { type InjectionKey, inject } from 'vue'

export const emuKey = Symbol() as InjectionKey<Client>
export const useEmulator = () => inject(emuKey)!

const 
