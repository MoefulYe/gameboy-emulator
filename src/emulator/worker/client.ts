import { computed, readonly, ref, shallowRef } from 'vue'
import { BASE_FREQ_HZ, DEFAULT_VOLUME, State } from '../constants'
import type { CartridgeInfo } from 'emulator/pkg/emulator'
import { Listener, type EventCallback, type ServerSideEvent } from './event/server_side_event'
import { Requester, type ClientSideEvent, type ReqArgs } from './event/client_side_event'
import { AudioReceiver } from './audio'

type CreateOption = {
  listenPort: MessagePort
  requestPort: MessagePort
  audioPort: MessagePort
  server: Worker
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
  private readonly audioReceiver: AudioReceiver
  private readonly server: Worker

  constructor({ listenPort, requestPort, audioPort, server }: CreateOption) {
    this.requester = new Requester(requestPort)
    this.listener = new Listener(listenPort)
    this.audioReceiver = new AudioReceiver(audioPort)
    this.server = server
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
