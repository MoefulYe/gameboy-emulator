import { onMounted, onUnmounted } from 'vue'

type EventTypes = string
export type EventData<
  Events extends Record<EventTypes, {}>,
  Event extends keyof Events
> = Events[Event]
type Packet<Events extends Record<EventTypes, {}>, Event extends keyof Events> = {
  type: Event
  data: EventData<Events, Event>
}

export type EventCallback<Args extends {}> = (args: Args) => void
type EventCallbacks<Args extends {}> = Set<EventCallback<Args>>
type EventCallbackMap<Events extends Record<EventTypes, {}>> = {
  [Event in keyof Events]: EventCallbacks<Events[Event]>
}

export class Emitter<Events extends Record<EventTypes, {}>> {
  constructor(private port: MessagePort) {}
  public emit<Event extends keyof Events>(
    type: Event,
    data: EventData<Events, Event>,
    transfer: Transferable[] = []
  ) {
    const packet = {
      type,
      data
    } satisfies Packet<Events, Event>
    this.port.postMessage(packet, transfer)
  }
}

export class Listener<Events extends Record<EventTypes, {}>> {
  public constructor(
    private port: MessagePort,
    private callbacks: Partial<EventCallbackMap<Events>> = {}
  ) {
    port.onmessage = (e: MessageEvent<Packet<Events, EventTypes>>) => {
      const { type, data } = e.data
      const callbacks = this.callbacks[type]
      if (callbacks === undefined) {
        return
      }
      for (const callback of callbacks) {
        callback(data)
      }
    }
  }
  public on<Event extends keyof Events>(event: Event, callback: EventCallback<Events[Event]>) {
    const entry = this.callbacks[event]
    if (entry !== undefined) {
      entry.add(callback)
    } else {
      const s = new Set<EventCallback<Events[Event]>>()
      s.add(callback)
      this.callbacks[event] = s
    }
  }
  public off<Event extends keyof Events>(event: Event, callback: EventCallback<Events[Event]>) {
    const entry = this.callbacks[event]
    entry?.delete(callback)
  }

  public use<Event extends keyof Events>(event: Event, callback: EventCallback<Events[Event]>) {
    onMounted(() => this.on(event, callback))
    onUnmounted(() => this.on(event, callback))
  }
}
