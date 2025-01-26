import { WasmEmulator } from 'emulator/pkg/emulator'
import { State, LogLevel, BASE_FREQ_HZ, VISUAL_FREQ_HZ, MS_PER_FRAME, Ok, Err } from '../constants'
import wasmInit from 'emulator/pkg'
import { every } from '@/utils/timer'
import { Responser, type ClientSideEvent, type Handlers } from './event/client_side_event'
import { Emitter } from './event/server_side_event'

export type CreateOption = {
  audioPort: MessagePort
  responsePort: MessagePort
  emitPort: MessagePort
}
type ConstructorOption = CreateOption & { core: WasmEmulator }

export class Server {
  core: WasmEmulator
  audioPort: MessagePort
  responser: Responser
  emitter: Emitter

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
    //注册回调
    self.emulatorLogCallback = (level: LogLevel, msg: string) => {
      console.log(level, msg)
    }
    self.emulatorSerialCallback = (byte: number) => {
      console.log('serial', byte)
    }
    await wasmInit()
    const core = new WasmEmulator()
    const worker = new Server({
      core,
      audioPort,
      responsePort,
      emitPort
    })
    worker.run()
    return worker
  }

  private clientSideEventHandlers(): Handlers<ClientSideEvent> {
    return {
      'load-rom': () => [
        {
          status: Ok,
          ret: undefined,
          err: undefined
        },
        []
      ],
      ping: () => [
        {
          status: Ok,
          ret: { msg: 'pong' }
        },
        []
      ],
      'set-canvas': ({ canvas }) => {
        const ctx = canvas.getContext('2d')!
        if (ctx === null) {
          return [{ status: Err, err: 'set canvas failed! fail to get context' }, []]
        }
        this.core.setCanvas(ctx)
        return [{ status: Ok, ret: undefined }, []]
      }
    }
  }
}
