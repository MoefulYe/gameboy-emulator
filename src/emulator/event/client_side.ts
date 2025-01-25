type _EventDef = {
  args: {}
  ret: unknown
  err: unknown
}
type EventTypes = string | number | symbol
type ReqArgs<
  Events extends Record<EventTypes, _EventDef>,
  Event extends EventTypes
> = Events[Event]['args']

type ReqPacket<Events extends Record<EventTypes, _EventDef>, Event extends EventTypes> = {
  type: Event
  id: number
  data: ReqArgs<Events, Event>
}

type RespRet<
  Events extends Record<EventTypes, _EventDef>,
  Event extends EventTypes
> = Events[Event]['ret']
type ClientEventRespErr<
  Events extends Record<EventTypes, _EventDef>,
  Event extends EventTypes
> = Events[Event]['err']
export const enum RespStatus {
  Ok,
  Err
}
type RespData<Events extends Record<EventTypes, _EventDef>, Event extends EventTypes> =
  | {
      status: RespStatus.Ok
      ret: RespRet<Events, Event>
    }
  | {
      status: RespStatus.Err
      ret: ClientEventRespErr<Events, Event>
    }
type RespPacket<Events extends Record<EventTypes, _EventDef>, Event extends EventTypes> = {
  id: number
  data: RespData<Events, Event>
}

type Handlers<Events extends Record<EventTypes, _EventDef>> = {
  [Event in keyof Events]: (
    args: ReqArgs<Events, Event>
  ) => [RespData<Events, Event>, Transferable[]]
}

export class Requester<Events extends Record<EventTypes, _EventDef>> {
  waiters = new Map<number, (resp: RespData<Events, EventTypes>) => void>()
  id = 0
  public constructor(private port: MessagePort) {
    port.onmessage = (e: MessageEvent<RespPacket<Events, EventTypes>>) => {
      const { id, data } = e.data
      const waiter = this.waiters.get(id)!
      this.waiters.delete(id)
      waiter(data)
    }
  }
  public async request<Event extends EventTypes>(
    type: Event,
    data: ReqArgs<Events, Event>,
    transfer: Transferable[] = []
  ): Promise<RespData<Events, Event>> {
    const id = this.id++
    const payload = {
      type,
      id,
      data
    } satisfies ReqPacket<Events, Event>
    const waiter = new Promise<RespData<Events, Event>>((r) => this.waiters.set(id, r))
    this.port.postMessage(payload, transfer)
    return await waiter
  }
}

export class Responser<Events extends Record<EventTypes, _EventDef>> {
  public constructor(
    private port: MessagePort,
    private handlers: Handlers<Events>
  ) {
    port.onmessage = (e: MessageEvent<ReqPacket<Events, EventTypes>>) => {
      const { type, id, data: args } = e.data
      const handler = handlers[type]
      const [data, transfers] = handler(args as any)
      const resp = { id, data } satisfies RespPacket<Events, EventTypes>
      port.postMessage(resp, transfers)
    }
  }
}
