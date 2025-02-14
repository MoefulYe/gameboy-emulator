import type { CartridgeInfo } from 'emulator/pkg/emulator'
import type { LogLevel } from '../constants'
import type { GameboyLayoutButtons } from '../input/gamepad'

export type ClientSideEvent = {
  'load-rom': {
    args: {
      rom: Uint8Array
    }
    ret: CartridgeInfo
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
}

export type ServerSideEvent = {
  log: {
    level: LogLevel
    msg: string
  }
  serial: {
    byte: number
  }
}
