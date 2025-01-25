export type EmulatorEvent = {
  log: [level: LogLevel, msg: string]
  serial: [byte: number]
  abort: [msg: string]
}

export type EmulatorEventType = keyof EmulatorEvent

export const createEmulatorEventEmitter = (): EventEmitter<EmulatorEvent> => {
  const emitter = createEventEmitter<EmulatorEvent>()
  window.emulatorLogCallback = (level: LogLevel, msg: string) => emitter.emit('log', level, msg)
  window.emulatorSerialCallback = (byte: number) => emitter.emit('serial', byte)
  return emitter
}

export type ClientSideEvent = {
  step: {
    args: {}
    ret: void
    err: void
  }
  loadRom: {
    args: { rom: Uint8Array }
    ret: void
    err: void
  }
}
export type ClientSideEventType = keyof ClientSideEvent
export type ClientEventHandlers = {
  [Event in ClientSideEventType]: (
    args: ClientEventRequestArgs<Event>
  ) => ClientEventResponse<Event>
}

export type ClientEventRequestArgs<Event extends ClientSideEventType> =
  ClientSideEvent[Event]['args']
type ClientEventRequestPacket<Event extends ClientSideEventType> = {
  type: Event
  id: number
  data: ClientEventRequestArgs<Event>
}

export type ClientEventResponseRet<Event extends ClientSideEventType> =
  ClientSideEvent[Event]['ret']
export type ClientEventResponseErr<Event extends ClientSideEventType> =
  ClientSideEvent[Event]['err']
export const enum ClientEventResponseStatus {
  Ok,
  Err
}
export type ClientEventResponse<Event extends ClientSideEventType> =
  | {
      status: ClientEventResponseStatus.Ok
      ret: ClientEventResponseRet<Event>
    }
  | {
      status: ClientEventResponseStatus.Err
      ret: ClientEventResponseErr<Event>
    }
export type ClientEventResponsePacket<Event extends ClientSideEventType> = {
  id: number
  data: ClientEventResponse<Event>
}

export class ClientEventRequester {
  waiters = new Map<number, (resp: Object) => void>()
  id = 0
  public constructor(private port: MessagePort) {
    port.onmessage = (e: MessageEvent<ClientEventResponsePacket<ClientSideEventType>>) => {
      const { id, ...data } = e.data
      const waiter = this.waiters.get(id)!
      this.waiters.delete(id)
      waiter(data)
    }
  }
  public async request<Event extends ClientSideEventType>(
    func: Event,
    args: ClientEventRequestArgs<Event>,
    transfer: Transferable[] = []
  ): Promise<ClientEventResponse<Event>> {
    const id = this.id++
    const payload = {
      func,
      id,
      args
    }
    const waiter = new Promise((r) => this.waiters.set(id, r))
    this.port.postMessage(payload, transfer)
    return (await waiter) as ClientEventResponse<Event>
  }
}

export class ClientEventResponser {
  constructor(
    private port: MessagePort,
    private handlers: ClientEventHandlers
  ) {
    port.onmessage = (e: MessageEvent<ClientEventRequestPacket<ClientSideEventType>>) => {}
  }
}

export type ServerSideEvent = {}
