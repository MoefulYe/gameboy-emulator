import type { GameboyLayoutButtons } from '../input/gamepad'

export type ClientSideEvent = {
  'load-rom': {
    args: {
      rom: Uint8Array
    }
    ret: undefined
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
}

export type ServerSideEvent = {
  hello: {}
  abort: {}
}
