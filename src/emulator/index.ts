import { type InjectionKey, inject } from 'vue'
import wasmInit, { WasmEmulator, Button as EmulatorButton } from 'emulator/pkg'
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
  private static readonly VISUAL_FREQ_HZ: number = 59.7

  public readonly freqScale = ref(1.0)
  public readonly volume = ref(50)
  public readonly state = ref(EmulatorState.Shutdown)
  public readonly cycles = ref(0)
  public readonly freqHz = computed(() => Emulator.BASE_FREQ_HZ * this.freqScale.value)
  public mode = 1
  private emitter: EventEmitter<EmulatorEvent>
  private canvansCtx?: CanvasRenderingContext2D
  private resumeSignal?: ResumeSignal
  private pauseWaiter?: PauseWaiter

  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.emitter = emitter
  }

  private tick(pendingTime: number): number {
    const current = performance.now()
    return 0
  }

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

  public useCanvas(canvas: ShallowRef<HTMLCanvasElement | undefined>) {
    onMounted(() => (this.canvansCtx = canvas.value!.getContext('2d') ?? undefined))
    onUnmounted(() => (this.canvansCtx = undefined))
  }

  public pause() {
    if (this.state.value === EmulatorState.Running) {
      this.pauseWaiter = new Promise((signal) => (this.resumeSignal = signal))
    }
  }

  public reset() {
    // TODO
    if (this.state.value === EmulatorState.Paused) {
      this.resumeSignal!()
    }
    super.reset()
    this.state.value = EmulatorState.Shutdown
    this.cycles.value = 0
    this.emitter.emit('log', LogLevel.Info, 'Emulator is reset')
  }

  public step() {
    if (this.state.value !== EmulatorState.Paused) {
      this.emitter.emit('log', LogLevel.Warn, 'Emulator only step when paused')
    }
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
        await this.pauseWaiter!
        continue
      } else if (s !== EmulatorState.Running) {
        break
      }
    }
  }
}

export const emuKey = Symbol() as InjectionKey<Emulator>
export const useEmulator = () => inject(emuKey)!

export { EmulatorButton }
