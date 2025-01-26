import { computed, readonly, ref, shallowRef } from 'vue'
import { BASE_FREQ_HZ, DEFAULT_VOLUME, State } from '../constants'
import type { CartridgeInfo } from 'emulator/pkg/emulator'
import { Listener, type EventCallback, type ServerSideEvent } from './server_side_event'
import { Requester, type ClientSideEvent, type ReqArgs } from './client_side_event'

type CreateOption = {
  listenePort: MessagePort
  requestePort: MessagePort
}

export class Client {
  public readonly volume = ref(DEFAULT_VOLUME)
  private readonly _state = ref(State.Shutdown)
  public readonly freqScale = ref(1.0)
  public readonly freqHz = computed(() => BASE_FREQ_HZ * this.freqScale.value)
  private readonly _cycles = ref(0)
  public readonly romInfo = shallowRef<CartridgeInfo>()

  public get state() {
    return readonly(this._state)
  }
  public get cycles() {
    return readonly(this._cycles)
  }

  private readonly requester: Requester
  private readonly listener: Listener

  constructor({ listenePort, requestePort }: CreateOption) {
    this.requester = new Requester(requestePort)
    this.listener = new Listener(listenePort)
  }

  public on<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.on(event, callback)
  }

  public off<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.off(event, callback)
  }

  public use<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.use(event, callback)
  }

  public request<Event extends keyof ClientSideEvent>(
    type: Event,
    data: ReqArgs<ClientSideEvent, Event>,
    transfer: Transferable[] = []
  ) {
    return this.requester.request(type, data, transfer)
  }
}
