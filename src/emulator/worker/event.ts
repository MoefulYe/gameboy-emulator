import type { CPUStateDump, CartInfo } from 'emulator/pkg/emulator'
import type { LogLevel, State } from '../constants'
import type { GameboyLayoutButtons } from '../input/gamepad/constants'

export type ClientSideEvent = {
  'load-rom': {
    args: {
      rom: Uint8Array
    }
    ret: CartInfo
    err: string
  }
  ping: {
    args: {
      msg: string
    }
    ret: string
    err: string
  }
  'set-canvas': {
    args: {
      canvas: OffscreenCanvas
    }
    ret: undefined
    err: string
  }
  'tile-canvas': {
    args: {
      canvas: OffscreenCanvas
    }
    ret: undefined
    err: string
  }
  'btn-action': {
    args: Readonly<GameboyLayoutButtons>
    ret: undefined
    err: undefined
  }
  'set-fscale': {
    args: number
    ret: undefined
    err: undefined
  }
  start: {
    args: {}
    ret: undefined
    err: string
  }
  pause: {
    args: {}
    ret: undefined
    err: undefined
  }
  step: {
    args: {}
    ret: undefined
    err: undefined
  }
  shutdown: {
    args: {}
    ret: undefined
    err: undefined
  }
  save: {
    args: {}
    ret: {
      data: Uint8Array
      state: State
    }
    err: undefined
  }
  load: {
    args: {
      data: Uint8Array
      state: State
    }
    ret: undefined
    err: undefined
  }
  'set-volume': {
    args: number
    ret: undefined
    err: undefined
  }
}

export type ServerSideEvent = {
  log: {
    level: LogLevel
    msg: string
  }[]
  update: {
    state?: State
    cycles?: number
    cpu?: CPUStateDump
    byte?: Uint8Array
    rom?: CartInfo | null
  }
}
