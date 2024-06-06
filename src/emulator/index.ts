import {
  type CPUState,
  type ClockCycle,
  type EmulatorErrorInfo
} from './../../emulator/pkg/emulator.d'
import { type InjectionKey, inject, reactive, shallowRef } from 'vue'
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
import wait from '@/utils/wait'
import { exclusive } from '@/utils/decorator'

const BASE_FREQ_HZ = 4_194_304
const VISUAL_FREQ_HZ = 59.7
const MS_PER_FRAME = 1000 / VISUAL_FREQ_HZ

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  public readonly freqScale = ref(1.0)
  public readonly volume = ref(50)
  public readonly state = ref(EmulatorState.Shutdown)
  public readonly cycles = ref<ClockCycle>(0)
  public readonly freqHz = computed(() => BASE_FREQ_HZ * this.freqScale.value)
  public readonly serialOutput = reactive<number[]>([])
  public readonly cpuTracedState = shallowRef<CPUState>()
  // TODO 添加切换gameboy执行模式的功能, SGB, CGB, DMG ...
  public mode = 1
  private emitter: EventEmitter<EmulatorEvent>
  private canvansCtx?: CanvasRenderingContext2D
  private running = false
  // TODO 添加状态统计, 实际帧率 已经运行的CPU周期, 以及其他的状态信息

  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.emitter = emitter
    this.on('serial', (data) => this.serialOutput.push(data))
  }

  private abort({ brief, msg }: EmulatorErrorInfo) {
    this.canvansCtx?.fillText(brief, 0, 0)
    this.emitter.emit('log', LogLevel.Error, msg)
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

  public step() {
    if (this.state.value !== EmulatorState.Paused) return
    const res = this._step()
    if (res.status === 'ok') {
      this.cycles.value += res.cycles
      this.cpuTracedState.value = res.cpu
    } else {
      this.state.value = EmulatorState.Aborted
      this.abort(res.info)
    }
  }

  @exclusive
  public async run() {
    // eslint-disable-next-line no-constant-condition
    while (true) {
      if (this.state.value !== EmulatorState.Running) break
      const ts = performance.now()
      const cycles = Math.floor(this.freqHz.value / VISUAL_FREQ_HZ)
      const res = this.update(cycles)
      this.cycles.value += res.cycles
      if (res.status === 'ok') {
        const diff = performance.now() - ts
        await wait(Math.max(0, MS_PER_FRAME - diff))
      } else if (res.status === 'break') {
        this.cpuTracedState.value = res.cpu
        this.state.value = EmulatorState.Paused
        break
      } else {
        this.state.value = EmulatorState.Aborted
        this.abort(res.info)
        break
      }
    }
  }
}

export const emuKey = Symbol() as InjectionKey<Emulator>
export const useEmulator = () => inject(emuKey)!

export { EmulatorButton }
