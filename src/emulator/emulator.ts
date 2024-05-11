import wasmInit, { WasmEmulator } from 'emulator/pkg/emulator'
import { type Ref, ref, type ComputedRef, computed } from 'vue'
import {
  useListener,
  type EventDispatcher,
  type EventEmitter,
  type EventListener
} from '@/utils/event'
import { EmulatorState } from './state'
import { createEmulatorEventEmitter, type EmulatorEvent, type EmulatorEventType } from './event'

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  private static readonly BASE_FREQ_HZ: number = 4_194_304

  private freqScale: Ref<number> = ref(1.0)
  private freqHz: ComputedRef<number> = computed(() => Emulator.BASE_FREQ_HZ * this.freqScale.value)
  private volume: Ref<number> = ref(50)
  private state: Ref<EmulatorState>
  private emitter: EventEmitter<EmulatorEvent>
  private canvansCtx?: CanvasRenderingContext2D

  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.state = ref(EmulatorState.Shutdown)
    this.emitter = emitter
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

  public screenshot(): void {
    if (this.canvansCtx !== undefined) {
      const imageData = this.canvansCtx.getImageData(0, 0, 160, 144)
    }
  }

  public useState(): Ref<EmulatorState> {
    return this.state
  }

  public useSpeedScale(): Ref<number> {
    return this.freqScale
  }

  public useSpeedHz(): ComputedRef<number> {
    return this.freqHz
  }

  public useVolume(): Ref<number> {
    return this.volume
  }
}

export const emulator = await Emulator.create()
