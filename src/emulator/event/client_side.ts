export type Events = {
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
type EventTypes = keyof Events
type Handlers = {
  [Event in EventTypes]: (args: ReqArgs<Event>) => [RespData<Event>, Transferable[]]
}

type ReqArgs<Event extends EventTypes> = Events[Event]['args']
type RequestPacket<Event extends EventTypes> = {
  type: Event
  id: number
  data: ReqArgs<Event>
}

type RespRet<Event extends EventTypes> = Events[Event]['ret']
type ClientEventResponseErr<Event extends EventTypes> = Events[Event]['err']
export const enum RespStatus {
  Ok,
  Err
}
type RespData<Event extends EventTypes> =
  | {
      status: RespStatus.Ok
      ret: RespRet<Event>
    }
  | {
      status: RespStatus.Err
      ret: ClientEventResponseErr<Event>
    }
type RespPacket<Event extends EventTypes> = {
  id: number
  data: RespData<Event>
}

export class Requester {
  waiters = new Map<number, (resp: RespData<EventTypes>) => void>()
  id = 0
  public constructor(private port: MessagePort) {
    port.onmessage = (e: MessageEvent<RespPacket<EventTypes>>) => {
      const { id, data } = e.data
      const waiter = this.waiters.get(id)!
      this.waiters.delete(id)
      waiter(data)
    }
  }
  public async request<Event extends EventTypes>(
    type: Event,
    data: ReqArgs<Event>,
    transfer: Transferable[] = []
  ): Promise<RespData<Event>> {
    const id = this.id++
    const payload = {
      type,
      id,
      data
    } satisfies RequestPacket<Event>
    const waiter = new Promise<RespData<Event>>((r) => this.waiters.set(id, r))
    this.port.postMessage(payload, transfer)
    return await waiter
  }
}

export class Responser {
  public constructor(
    private port: MessagePort,
    private handlers: Handlers
  ) {
    port.onmessage = (e: MessageEvent<RequestPacket<EventTypes>>) => {
      const { type, id, data: args } = e.data
      const handler = handlers[type]
      const [data, transfers] = handler(args as any)
      const resp = { id, data } satisfies RespPacket<EventTypes>
      port.postMessage(resp, transfers)
    }
  }
}
