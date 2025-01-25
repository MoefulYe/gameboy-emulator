import { WasmEmulator } from 'emulator/pkg/emulator'
import { EmulatorState } from '../state'
import wasmInit from 'emulator/pkg'
import type { LogLevel } from '../event'
import { every } from '@/utils/timer'

const BASE_FREQ_HZ = 4_194_304
const VISUAL_FREQ_HZ = 59.7
const MS_PER_FRAME = 1000 / VISUAL_FREQ_HZ

type CreateOption = {
  canvasCtx: OffscreenCanvasRenderingContext2D
  audioChan: MessageChannel
  clientSideEventChan: MessageChannel
  serverSideEventChan: MessageChannel
}
type ConstructorOption = Omit<CreateOption & { core: WasmEmulator }, 'canvasCtx'>

export class EmulatorServer {
  core: WasmEmulator
  audioChan: MessageChannel
  clientSideEventChan: MessageChannel
  serverSideEventChan: MessageChannel
  freqScale = 1.0
  volume = 50
  state = EmulatorState.Shutdown
  cycles = 0
  get freqHz() {
    return BASE_FREQ_HZ * this.freqScale
  }
  mode = 1

  private constructor({
    core,
    audioChan,
    clientSideEventChan,
    serverSideEventChan
  }: ConstructorOption) {
    this.core = core
    this.audioChan = audioChan
    this.clientSideEventChan = clientSideEventChan
    this.serverSideEventChan = serverSideEventChan
  }

  private run() {
    every(() => {
      if (this.state !== EmulatorState.Running) return
      const cycles = Math.floor(this.freqHz / VISUAL_FREQ_HZ)
      const res = this.core.update(cycles)
      this.cycles += res.cycles
      if (res.status === 'ok') {
        return
      } else {
        this.state = EmulatorState.Aborted
        return
      }
    }, MS_PER_FRAME)
  }
  public static async create({
    canvasCtx,
    audioChan,
    clientSideEventChan,
    serverSideEventChan
  }: CreateOption) {
    //注册回调
    self.emulatorLogCallback = (level: LogLevel, msg: string) => {
      console.log(level, msg)
    }
    self.emulatorSerialCallback = (byte: number) => {
      console.log('serial', byte)
    }
    await wasmInit()
    const core = new WasmEmulator(canvasCtx)
    const worker = new EmulatorServer({
      core,
      audioChan,
      clientSideEventChan,
      serverSideEventChan
    })
    worker.run()
    return worker
  }
}
