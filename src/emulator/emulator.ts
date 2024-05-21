import wasmInit, { WasmEmulator } from 'emulator/pkg/emulator'
import { ref, computed, onMounted, onUnmounted, type ShallowRef } from 'vue'
import {
  useListener,
  type EventDispatcher,
  type EventEmitter,
  type EventListener
} from '@/utils/event'
import { EmulatorState } from './state'
import {
  LogLevel,
  createEmulatorEventEmitter,
  type EmulatorEvent,
  type EmulatorEventType
} from './event'

export type ResumeSignal = () => void
export type PauseWaiter = Promise<void>

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  private static readonly BASE_FREQ_HZ: number = 4_194_304

  private freqScale = ref(1.0)
  private volume = ref(50)
  private state = ref(EmulatorState.Shutdown)
  private freqHz = computed(() => Emulator.BASE_FREQ_HZ * this.freqScale.value)
  private emitter: EventEmitter<EmulatorEvent>
  private canvansCtx?: CanvasRenderingContext2D
  private resumeSignal?: ResumeSignal
  private PauseWaiter?: PauseWaiter
  private cycles = ref(0)

  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.emitter = emitter
  }

  private tick()

  public static async create(): Promise<Emulator> {
    const emitter = createEmulatorEventEmitter()
    await wasmInit()
    WasmEmulator.initLogger()
    return new Emulator(emitter)
  }

  public on<Event extends EmulatorEventType>(
    event: Event,
    listener: EventListener<EmulatorEvent[Event]>
  ) {
    this.emitter.on(event, listener)
  }

  public off<Event extends EmulatorEventType>(
    event: Event,
    listener: EventListener<EmulatorEvent[Event]>
  ) {
    this.emitter.off(event, listener)
  }

  public useListener<Event extends EmulatorEventType>(
    event: Event,
    listener: EventListener<EmulatorEvent[Event]>
  ) {
    useListener(this.emitter, event, listener)
  }

  public screenshot() {
    if (this.canvansCtx !== undefined) {
      const imageData = this.canvansCtx.getImageData(0, 0, 160, 144)
    }
  }

  public useState() {
    return this.state
  }

  public useSpeedScale() {
    return this.freqScale
  }

  public useSpeedHz() {
    return this.freqHz
  }

  public useVolume() {
    return this.volume
  }

  public useCanvas(canvas: ShallowRef<HTMLCanvasElement | undefined>) {
    onMounted(() => (this.canvansCtx = canvas.value!.getContext('2d') ?? undefined))
    onUnmounted(() => (this.canvansCtx = undefined))
  }

  public pause() {
    if (this.state.value === EmulatorState.Running) {
      this.PauseWaiter = new Promise((signal) => (this.resumeSignal = signal))
    }
  }

  public reset() {
    if (this.state.value === EmulatorState.Paused) {
      this.resumeSignal!()
    }
    super.reset()
    this.state.value = EmulatorState.Shutdown
    this.cycles.value = 0
    this.emitter.emit('log', LogLevel.Info, 'Emulator is reset')
  }

  public async start() {
    if (this.state.value === EmulatorState.Aborted) {
      this.emitter.emit('log', LogLevel.Warn, 'Emulator is aborted. reset emulator first')
      return
    }
    const pending = 0
    // eslint-disable-next-line no-constant-condition
    while (true) {
      const s = this.state.value
      if (s === EmulatorState.Paused) {
        await this.PauseWaiter!
        continue
      } else if (s !== EmulatorState.Running) {
        break
      }
    }
  }
}

export const emulator = await Emulator.create()
