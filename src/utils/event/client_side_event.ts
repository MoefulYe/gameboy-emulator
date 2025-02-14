import { withResolver } from '@/utils/promise'

export const enum Status {
  Ok,
  Err
}
type _EventDef = {
  args: {}
  ret: unknown
  err: unknown
}
type EventTypes = string | number | symbol
export type ReqArgs<
  Events extends Record<EventTypes, _EventDef>,
  Event extends EventTypes
> = Events[Event]['args']

type ReqPacket<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> = {
  type: Event
  id: number
  data: ReqArgs<Events, Event>
}

type RespRet<
  Events extends Record<EventTypes, _EventDef>,
  Event extends keyof Events
> = Events[Event]['ret']
type ClientEventRespErr<
  Events extends Record<EventTypes, _EventDef>,
  Event extends keyof Events
> = Events[Event]['err']
type RespData<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> =
  | {
      status: Status.Ok
      ret: RespRet<Events, Event>
    }
  | {
      status: Status.Err
      err: ClientEventRespErr<Events, Event>
    }
type RespPacket<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> = {
  id: number
  data: RespData<Events, Event>
}

type SyncHandler<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> = (
  args: ReqArgs<Events, Event>
) => [RespData<Events, Event>, Transferable[]]
type AsyncHandler<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> = (
  args: ReqArgs<Events, Event>
) => Promise<[RespData<Events, Event>, Transferable[]]>
export type Handler<Events extends Record<EventTypes, _EventDef>, Event extends keyof Events> =
  | SyncHandler<Events, Event>
  | AsyncHandler<Events, Event>

export type Handlers<Events extends Record<EventTypes, _EventDef>> = {
  [Event in keyof Events]: Handler<Events, Event>
}

export class Requester<Events extends Record<EventTypes, _EventDef>> {
  private waiters = new Map<number, (resp: RespData<Events, EventTypes>) => void>()
  private id = 0
  public constructor(private port: MessagePort) {
    port.onmessage = (e: MessageEvent<RespPacket<Events, EventTypes>>) => {
      const { id, data } = e.data
      const waiter = this.waiters.get(id)!
      this.waiters.delete(id)
      waiter(data)
    }
  }
  public async request<Event extends keyof Events>(
    type: Event,
    data: ReqArgs<Events, Event>,
    transfer: Transferable[] = []
  ): Promise<RespData<Events, Event>> {
    const id = this.id
    this.id++
    const payload = {
      type,
      id,
      data
    } satisfies ReqPacket<Events, Event>
    const [waiter, resolver] = withResolver<RespData<Events, Event>>()
    this.waiters.set(id, resolver)
    this.port.postMessage(payload, transfer)
    return await waiter
  }
}

export class Responser<Events extends Record<EventTypes, _EventDef>> {
  public constructor(
    private port: MessagePort,
    private handlers: Handlers<Events>
  ) {
    port.onmessage = async (e: MessageEvent<ReqPacket<Events, keyof Events>>) => {
      const { type, id, data: args } = e.data
      const handler = handlers[type]
      const res = handler(args as any)
      if (res instanceof Promise) {
        const [data, transfers] = await res
        const resp = { id, data } satisfies RespPacket<Events, keyof Events>
        port.postMessage(resp, transfers)
      } else {
        const [data, transfers] = res
        const resp = { id, data } satisfies RespPacket<Events, keyof Events>
        port.postMessage(resp, transfers)
      }
    }
  }
}
