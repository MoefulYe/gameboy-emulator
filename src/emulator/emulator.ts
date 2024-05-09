import wasmInit, { WasmEmulator } from 'emulator/pkg/emulator'
import { type Ref, ref } from 'vue'
import {
  useListener,
  type EventDispatcher,
  type EventEmitter,
  type EventListener
} from '@/utils/event'
import { EmulatorState } from './state'
import { createEmulatorEventEmitter, type EmulatorEvent, type EmulatorEventType } from './event'

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  private state: Ref<EmulatorState>
  private emitter: EventEmitter<EmulatorEvent>
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

  public useState(): Ref<EmulatorState> {
    return this.state
  }
}

export const emulator = await Emulator.create()
