import { WasmEmulator } from 'emulator/pkg/emulator'
import { State, LogLevel, BASE_FREQ_HZ, VISUAL_FREQ_HZ, MS_PER_FRAME } from '../constants'
import wasmInit from 'emulator/pkg'
import { every } from '@/utils/timer'
import { NONE, Responser, Right, Throw } from '@/utils/event/client_side_event'
import { Emitter, type EventData } from '@/utils/event/server_side_event'
import type { ClientSideEvent, ServerSideEvent } from './event'
import { AudioSender } from '../output/audio'

export type CreateOption = {
  audioPort: MessagePort
  responsePort: MessagePort
  emitPort: MessagePort
  freqScale: number
}

type Handler<Event extends keyof ClientSideEvent> =
  import('@/utils/event/client_side_event').Handler<ClientSideEvent, Event>
type Handlers = import('@/utils/event/client_side_event').Handlers<ClientSideEvent>

export class Server {
  responser: Responser<ClientSideEvent>
  freqScale = 1.0
  state = State.Shutdown
  get freqHz() {
    return BASE_FREQ_HZ * this.freqScale
  }
  updateInput = {
    btns: 0
  }

  private constructor(
    private core: WasmEmulator,
    private audio: AudioSender,
    private emitter: Emitter<ServerSideEvent>,
    responsePort: MessagePort
  ) {
    const handlers = this.handlers()
    this.responser = new Responser(responsePort, handlers)
    this.poll()
  }

  private emit<Event extends keyof ServerSideEvent>(
    event: Event,
    data: EventData<ServerSideEvent, Event>,
    transfers: Transferable[] = []
  ) {
    this.emitter.emit(event, data, transfers)
  }

  public static async create({ audioPort, emitPort, responsePort, freqScale }: CreateOption) {
    const audio = new AudioSender(audioPort)
    const emitter = new Emitter<ServerSideEvent>(emitPort)
    self.emulatorLogCallback = (level, msg) => emitter.emit('log', { level: level as any, msg })
    self.emulatorSerialCallback = (byte) => emitter.emit('update', { byte })
    await wasmInit()
    WasmEmulator.initLogger()
    const core = new WasmEmulator()
    const server = new Server(core, audio, emitter, responsePort)
    server.freqScale = freqScale
    return server
  }

  private handleStep(): Handler<'step'> {
    return () => {
      if (this.state === State.Aborted) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'step when aborted! restart first!'
        })
      } else {
        if (this.state !== State.Paused) {
          this.state = State.Paused
          this.emit('update', { state: State.Paused })
        }
        this.update(1)
      }
      return NONE
    }
  }

  private poll() {
    every(() => {
      if (this.state !== State.Running) return
      const toExec = Math.floor(this.freqHz / VISUAL_FREQ_HZ)
      this.update(toExec)
    }, MS_PER_FRAME)
  }

  private update(cyclesToExec: number) {
    const now = Date.now()
    const { err, cpu, cycles } = this.core.update({
      ...this.updateInput,
      cycles: cyclesToExec,
      timestamp: now
    })
    if (err === null) {
      this.emit('update', { cpu, cycles })
    } else {
      this.state = State.Aborted
      this.emit('update', { state: State.Aborted, cycles, cpu })
      this.emit('log', { level: LogLevel.Error, msg: err })
    }
  }

  private handlers(): Handlers {
    return {
      'load-rom': this.handleLoadRom(),
      ping: this.handlePing(),
      'set-canvas': this.handleSetCanvas(),
      'tile-canvas': this.handleSetTileCanvas(),
      'btn-action': this.handleBtnAction(),
      'set-fscale': this.handleSetFScale(),
      start: this.handleStart(),
      pause: this.handlePause(),
      step: this.handleStep(),
      shutdown: this.handleShutdown(),
      save: this.handleSave(),
      load: this.handleLoad()
    }
  }

  private handleLoadRom(): Handler<'load-rom'> {
    return ({ rom }) => {
      const now = Date.now()
      const res = this.core.loadCart(rom, now)
      if (res.status === 'ok') {
        const { info } = res
        return Right(info)
      } else {
        const { msg } = res
        return Throw(msg)
      }
    }
  }

  private handlePing(): Handler<'ping'> {
    return () => {
      return Right('emulator copyright (c) 2024 moefulye')
    }
  }

  private handleSetCanvas(): Handler<'set-canvas'> {
    return ({ canvas }) => {
      const ctx = canvas.getContext('2d')
      if (ctx === null) {
        return Throw('set canvas failed! fail to get context')
      }
      this.core.setScreenCanvas(ctx)
      return NONE
    }
  }

  private handleSetTileCanvas(): Handler<'tile-canvas'> {
    return ({ canvas }) => {
      const ctx = canvas.getContext('2d')
      if (ctx === null) {
        return Throw('set tiles canvas failed! fail to get context')
      }
      this.core.setTilesCanvas(ctx)
      return NONE
    }
  }

  private handleBtnAction(): Handler<'btn-action'> {
    return (btns) => {
      this.updateInput.btns = btns
      return NONE
    }
  }

  private handleSetFScale(): Handler<'set-fscale'> {
    return (scale) => {
      this.freqScale = scale
      return NONE
    }
  }

  private handleStart(): Handler<'start'> {
    return () => {
      if (this.state === State.Aborted) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been aborted! restart first!'
        })
      } else if (this.state === State.Running) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been started!'
        })
      } else {
        this.state = State.Running
        this.emit('update', { state: State.Running })
      }
      return NONE
    }
  }

  private handlePause(): Handler<'pause'> {
    return () => {
      if (this.state === State.Aborted) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been aborted! restart first!'
        })
      } else if (this.state === State.Shutdown) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been shutdown! boot first!'
        })
      } else if (this.state === State.Paused) {
        this.emit('log', {
          level: LogLevel.Warn,
          msg: 'emulator has been paused!'
        })
      } else {
        this.state = State.Paused
        this.emit('update', { state: State.Paused })
      }
      return NONE
    }
  }

  private handleShutdown(): Handler<'shutdown'> {
    return () => {
      this.core.reset()
      this.state = State.Shutdown
      this.emit('update', {
        state: State.Shutdown,
        cycles: 0,
        rom: null
      })
      this.emit('log', {
        level: LogLevel.Info,
        msg: 'emu has been reset'
      })
      return NONE
    }
  }

  private handleSave(): Handler<'save'> {
    return () => {
      const data = this.core.save()
      if (data !== undefined) {
        return Right(
          {
            data,
            state: this.state
          },
          [data.buffer]
        )
      } else {
        return Throw(undefined)
      }
    }
  }

  private handleLoad(): Handler<'load'> {
    return ({ data, state }) => {
      const ok = this.core.load(data)
      if (ok) {
        this.state = state
        this.emit('update', {
          state
        })
        return Right(undefined)
      } else {
        this.state = State.Aborted
        this.emit('update', {
          state: State.Aborted
        })
        return Throw(undefined)
      }
    }
  }
}
