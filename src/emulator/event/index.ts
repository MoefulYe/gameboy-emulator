import { Requester, Responser } from './client_side'
import { Emitter, Listener } from './server_side'

export type ClientSideEvent = {
  loadRom: {
    args: {}
    ret: void
    err: void
  }
  ping: {
    args: {}
    ret: void
    err: void
  }
}

export type ServerSideEvent = {
  hello: {}
  abort: {}
}

export type ClientSideEventRequester = Requester<ClientSideEvent>
export type ClientSideEventResponser = Responser<ClientSideEvent>
export type ServerSideEventEmitter = Emitter<ServerSideEvent>
export type ServerSideEventListener = Listener<ServerSideEvent>
