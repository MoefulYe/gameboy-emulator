import type { CartridgeInfo } from 'emulator/pkg/emulator'
import { readonly, ref, shallowRef } from 'vue'
import { State } from './constants'

export class Stat {
  public readonly romInfo = shallowRef<CartridgeInfo>()
  private readonly _cycles = ref(0)
  public get cycles() {
    return readonly(this._cycles)
  }
  private readonly _state = ref(State.Shutdown)
  public get state() {
    return readonly(this._state)
  }
}

export const useStat = () => new Stat()
