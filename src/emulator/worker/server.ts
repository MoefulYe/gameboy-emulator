import { WasmEmulator } from 'emulator/pkg/emulator'
import { State, LogLevel, BASE_FREQ_HZ, VISUAL_FREQ_HZ, MS_PER_FRAME, Ok, Err } from '../constants'
import wasmInit from 'emulator/pkg'
import { every } from '@/utils/timer'
import { Responser } from '@/utils/event/client_side_event'
import { Emitter } from '@/utils/event/server_side_event'
import type { ClientSideEvent, ServerSideEvent } from './event'
import type { GameboyLayoutButton } from '../input/gamepad'

export type CreateOption = {
  audioPort: MessagePort
  responsePort: MessagePort
  emitPort: MessagePort
}
type ConstructorOption = CreateOption & { core: WasmEmulator }
type Handler<Event extends keyof ClientSideEvent> =
  import('@/utils/event/client_side_event').Handler<ClientSideEvent, Event>
type Handlers = import('@/utils/event/client_side_event').Handlers<ClientSideEvent>
export class Server {
  core: WasmEmulator
  audioPort: MessagePort
  responser: Responser<ClientSideEvent>
  emitter: Emitter<ServerSideEvent>

  freqScale = 1.0
  state = State.Shutdown
  cycles = 0
  get freqHz() {
    return BASE_FREQ_HZ * this.freqScale
  }
  mode = 1

  private constructor({ core, audioPort, responsePort, emitPort }: ConstructorOption) {
    this.core = core
    this.audioPort = audioPort
    this.emitter = new Emitter(emitPort)
    const handlers = this.clientSideEventHandlers()
    this.responser = new Responser(responsePort, handlers)
  }

  private run() {
    every(() => {
      if (this.state !== State.Running) return
      const cycles = Math.floor(this.freqHz / VISUAL_FREQ_HZ)
      const res = this.core.update(cycles)
      this.cycles += res.cycles
      if (res.status === 'ok') {
        return
      } else {
        this.state = State.Aborted
        return
      }
    }, MS_PER_FRAME)
  }

  public static async create({ audioPort, emitPort, responsePort }: CreateOption) {
    await wasmInit()
    const core = new WasmEmulator()
    const worker = new Server({
      core,
      audioPort,
      responsePort,
      emitPort
    })
    const emitter = worker.emitter
    //注册回调
    self.emulatorLogCallback = (level: LogLevel, msg: string) => {
      emitter.emit('log', { level, msg })
    }
    self.emulatorSerialCallback = (byte: number) => {
      emitter.emit('serial', { byte })
    }
    worker.run()
    return worker
  }

  private clientSideEventHandlers(): Handlers {
    return {
      'load-rom': this.handleLoadRom(),
      ping: this.handlePing(),
      'set-canvas': this.handleSetCanvas(),
      'btn-action': this.handleBtnAction(),
      'set-fscale': this.handleSetFScale(),
      start: this.handleStart()
    }
  }

  private handleLoadRom(): Handler<'load-rom'> {
    return ({ rom }) => {
      const res = this.core.pluginCart(rom)
      if (res.status === 'ok') {
        const { info } = res
        return [{ status: Ok, ret: info }, []]
      } else {
        const { msg } = res
        return [{ status: Err, err: msg }, []]
      }
    }
  }

  private handlePing(): Handler<'ping'> {
    return () => {
      return [{ status: Ok, ret: 'Emulator Copyright (C) 2024 Moefulye' }, []]
    }
  }

  private handleSetCanvas(): Handler<'set-canvas'> {
    return ({ canvas }) => {
      const ctx = canvas.getContext('2d')
      console.log('set canvas')
      if (ctx === null) {
        return [{ status: Err, err: 'set canvas failed! fail to get context' }, []]
      }
      this.core.setCanvas(ctx)
      return [{ status: Ok, ret: undefined }, []]
    }
  }

  private handleBtnAction(): Handler<'btn-action'> {
    return (btns) => {
      let u8 = 0
      for (let i = 0; i < 8; i++) {
        const pressed = btns[i as GameboyLayoutButton] ? 1 : 0
        u8 |= pressed << i
      }
      this.core.setButtons(u8)
      return [{ status: Ok, ret: undefined }, []]
    }
  }

  private handleSetFScale(): Handler<'set-fscale'> {
    return (scale) => {
      this.freqScale = scale
      return [{ status: Ok, ret: undefined }, []]
    }
  }

  private handleStart(): Handler<'start'> {
    return () => {
      if (this.state === State.Aborted) {
        return [{ status: Err, err: 'cannot start when aborted! Restart First!' }, []]
      }
      if (this.state === State.Running) {
        this.emitter.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been starting...'
        })
        return [{ status: Ok, ret: undefined }, []]
      }
      this.state = State.Running
      return [{ status: Ok, ret: undefined }, []]
    }
  }
}
