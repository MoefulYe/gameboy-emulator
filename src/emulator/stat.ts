import { type CPUStateDump, type CartridgeInfo } from 'emulator/pkg/emulator'
import { readonly, ref, shallowRef } from 'vue'
import { State } from './constants'

export class Stat {
  public readonly rom = shallowRef<CartridgeInfo>()
  private readonly _cycles = ref(0)
  public get cycles() {
    return readonly(this._cycles)
  }
  private readonly _state = ref(State.Shutdown)
  public get state() {
    return readonly(this._state)
  }
  public readonly serialBytes = ref('')
  public readonly cpu = shallowRef<CPUStateDump>({
    ime: false,
    halted: false,
    a: 0,
    f: 0,
    b: 0,
    c: 0,
    d: 0,
    e: 0,
    h: 0,
    l: 0,
    af: 0,
    bc: 0,
    de: 0,
    hl: 0,
    pc: 0,
    sp: 0,
    zeroFlag: false,
    negativeFlag: false,
    halfFlag: false,
    carryFlag: false,
    inst: 'UNKNOWN',
    threeWordsAtPc: [0, 0, 0]
  })
}

export const useStat = () => new Stat()
