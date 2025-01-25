import type { LogLevel } from '../log'

export type Events = {
  log: [level: LogLevel, msg: string]
  abort: [msg: string]
}
type EventTypes = keyof Events
type EventData<Event extends EventTypes> = Events[Event]
type Packet<Event extends EventTypes> = {
  type: Event
  data: EventData<Event>
}

export class Emitter {
  constructor(private port: MessagePort) {}
  public emit<Event extends EventTypes>(
    type: Event,
    data: EventData<Event>,
    transfer: Transferable[] = []
  ) {
    const packet = {
      type,
      data
    } satisfies Packet<Event>
    this.port.postMessage(packet, transfer)
  }
}

type EventCallback<Args extends unknown[]> = (...args: Args) => unknown
type EventCallbacks<Event extends EventTypes> = Array<EventCallback<Events[Event]>>
type EventCallbackMap = {
  [Event in EventTypes]: EventCallbacks<Event>
}

export class Listener {
  public constructor(
    private port: MessagePort,
    private listeners: Partial<EventCallbackMap> = {}
  ) {}
  public on<Event extends EventTypes>(event: Event, callback: EventCallback<Events[Event]>) {
    const entry = this.listeners[event]
    if (entry !== undefined) {
      entry.push(callback)
    } else {
      this.listeners[event] = [callback]
    }
  }
}
