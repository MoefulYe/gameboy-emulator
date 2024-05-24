import { type InjectionKey, inject, reactive } from 'vue'
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

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  private static readonly BASE_FREQ_HZ: number = 4_194_304
  private static readonly VISUAL_FREQ_HZ: number = 59.7

  public readonly freqScale = ref(1.0)
  public readonly volume = ref(50)
  public readonly state = ref(EmulatorState.Shutdown)
  public readonly cycles = ref(0)
  public readonly freqHz = computed(() => Emulator.BASE_FREQ_HZ * this.freqScale.value)
  public readonly serialOutput = reactive<number[]>([])
  // TODO 添加切换gameboy执行模式的功能, SGB, CGB, DMG ...
  public mode = 1
  private emitter: EventEmitter<EmulatorEvent>
  private canvansCtx?: CanvasRenderingContext2D
  private running = false
  // TODO 添加状态统计, 实际帧率 已经运行的CPU周期, 以及其他的状态信息
  private stats = 0

  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.emitter = emitter
    this.on('serial', (data) => this.serialOutput.push(data))
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

  public pause() {}

  public reset() {
    // TODO
  }

  // public step() {}

  public async start() {
    // 防止多个同时执行的start函数
    if (this.running) {
      return
    }
    this.running = true

    this.running = false
  }
}

export const emuKey = Symbol() as InjectionKey<Emulator>
export const useEmulator = () => inject(emuKey)!

export { EmulatorButton }
