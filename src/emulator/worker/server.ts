import { WasmEmulator } from 'emulator/pkg/emulator'
import { State, LogLevel, MS_PER_FRAME, CYCLES_PER_FRAME } from '../constants'
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
  volume: number
}

type Handler<Event extends keyof ClientSideEvent> =
  import('@/utils/event/client_side_event').Handler<ClientSideEvent, Event>
type Handlers = import('@/utils/event/client_side_event').Handlers<ClientSideEvent>

export class Server {
  responser: Responser<ClientSideEvent>
  state = State.Shutdown
  lastUpdated = 0
  lastCycles = 0
  updateInput = {
    btns: 0
  }

  private constructor(
    private core: WasmEmulator,
    public audioSender: AudioSender,
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

  public static async create({
    audioPort,
    emitPort,
    responsePort,
    freqScale,
    volume
  }: CreateOption) {
    const audioSender = new AudioSender(audioPort)
    const emitter = new Emitter<ServerSideEvent>(emitPort)
    self.emulatorLogCallback = (logs) => {
      emitter.emit('log', logs)
    }
    self.emulatorSerialCallback = (bytes) => {
      const $bytes = new Uint8Array(bytes)
      emitter.emit('update', { byte: $bytes }, [$bytes.buffer])
    }
    self.emulatorAudioCallback = (left, right) => {
      audioSender.send(left, right)
    }
    await wasmInit()
    WasmEmulator.initLogger()
    const core = new WasmEmulator(freqScale, volume * 0.01)
    const server = new Server(core, audioSender, emitter, responsePort)
    return server
  }

  private log(level: LogLevel, msg: string) {
    this.emit('log', [
      {
        level,
        msg
      }
    ])
  }

  private handleStep(): Handler<'step'> {
    return () => {
      if (this.state === State.Aborted) {
        this.log(LogLevel.Warn, 'step when aborted! restart first!')
      } else {
        if (this.state !== State.Paused) {
          this.state = State.Paused
          this.emit('update', { state: State.Paused })
        }
        this._step()
      }
      return NONE
    }
  }

  private poll() {
    every(() => {
      if (this.state !== State.Running) return
      this.update()
    }, MS_PER_FRAME)
  }

  private updateFPSandCycles(cycles: number) {
    const execed = cycles - this.lastCycles
    const now = Date.now()
    const elapsed = (now - this.lastUpdated) / 1000
    this.lastCycles = cycles
    this.lastUpdated = now
    const fps = execed / CYCLES_PER_FRAME / elapsed
    return fps
  }

  private update() {
    const now = Date.now()
    const { err, cpu, cycles } = this.core.update({
      ...this.updateInput,
      timestamp: now
    })
    const fps = this.updateFPSandCycles(cycles)
    if (err === null) {
      this.emit('update', { cpu, cycles, fps })
    } else {
      this.state = State.Aborted
      this.emit('update', { state: State.Aborted, cycles, cpu, fps })
      this.log(LogLevel.Error, err)
    }
  }

  private _step() {
    const now = Date.now()
    const { err, cpu, cycles } = this.core.step({
      ...this.updateInput,
      timestamp: now
    })
    this.lastCycles = cycles
    if (err === null) {
      this.emit('update', { cpu, cycles, fps: 0 })
    } else {
      this.state = State.Aborted
      this.emit('update', { state: State.Aborted, cycles, cpu, fps: 0 })
      this.log(LogLevel.Error, err)
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
      load: this.handleLoad(),
      'set-volume': this.handleSetVolume()
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
      this.core.setFreqScale(scale)
      return NONE
    }
  }

  private handleStart(): Handler<'start'> {
    return () => {
      if (this.state === State.Aborted) {
        this.log(LogLevel.Warn, 'emulator has been aborted! restart first!')
      } else if (this.state === State.Running) {
        this.log(LogLevel.Warn, 'emulator has been started!')
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
        this.log(LogLevel.Warn, 'emulator has been aborted! restart first!')
      } else if (this.state === State.Shutdown) {
        this.log(LogLevel.Warn, 'emulator has been shutdown! boot first!')
      } else if (this.state === State.Paused) {
        this.log(LogLevel.Warn, 'emulator has been paused!')
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
      this.log(LogLevel.Info, 'emu has been reset')
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

  private handleSetVolume(): Handler<'set-volume'> {
    return (volume) => {
      this.core.setVolume(volume * 0.01)
      return NONE
    }
  }
}
