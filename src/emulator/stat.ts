import { type CPUStateDump, type CartInfo } from 'emulator/pkg/emulator'
import { computed, shallowReactive, shallowRef, type ComputedRef } from 'vue'
import { State, VISUAL_FREQ_HZ, type SaveMetadata } from './constants'
import type { Config } from './config'

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
  public readonly rom = shallowRef<CartInfo | null>(null)
  public readonly cycles = shallowRef(0)
  public readonly state = shallowRef(State.Shutdown)
  public readonly serialBytes = shallowReactive([] as number[])
  public readonly cpu = shallowRef<CPUStateDump>(CPU_STATE_INIT)
  public readonly actualFPS = shallowRef(0)
  public readonly saveMetaData = shallowRef<SaveMetadata>()
  public readonly desiredFPS: ComputedRef<number>
  public constructor(config: Config) {
    this.desiredFPS = computed(() => VISUAL_FREQ_HZ * config.freqScale.value)
  }
}

export const useStat = (config: Config) => new Stat(config)
