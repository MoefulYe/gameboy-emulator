import wasmInit, { WasmEmulator } from 'emulator/pkg/emulator'
import { type ShallowRef, shallowRef } from 'vue'
import { type EventDispatcher, type EventEmitter, type EventListener } from '@/event'
import { StateType, type EmulatorState } from './state'
import { createEmulatorEventEmitter, type EmulatorEvent, type EmulatorEventType } from './event'

export class Emulator extends WasmEmulator implements EventDispatcher<EmulatorEvent> {
  private state: ShallowRef<EmulatorState>
  private emitter: EventEmitter<EmulatorEvent>
  private constructor(emitter: EventEmitter<EmulatorEvent>) {
    super()
    this.state = shallowRef({
      state: StateType.Stopped
    })
    this.emitter = emitter
  }

  public getState(): ShallowRef<EmulatorState> {
    return this.state
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
}

export default await Emulator.create()
