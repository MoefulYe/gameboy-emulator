import { type CPUStateDump, type CartridgeInfo } from 'emulator/pkg/emulator'
import { reactive, ref, shallowRef } from 'vue'
import { State } from './constants'

const CPU_STATE_INIT = {
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
} as const satisfies CPUStateDump

export class Stat {
  public readonly rom = shallowRef<CartridgeInfo>()
  public readonly cycles = ref(0)
  public readonly state = ref(State.Shutdown)
  public readonly serialBytes = reactive([] as number[])
  public readonly cpu = shallowRef<CPUStateDump>(CPU_STATE_INIT)
}

export const useStat = () => new Stat()
