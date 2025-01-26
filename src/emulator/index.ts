import { Client } from './worker/client'
import { type InjectionKey, inject } from 'vue'
import Server from './worker/worker?worker'
import { Err } from './constants'

export const emuKey = Symbol() as InjectionKey<Client>
export const useEmulator = () => inject(emuKey)!

export const createEmulator = async () => {
  const worker = new Server()
  const audioChan = new MessageChannel()
  const clientEventChan = new MessageChannel()
  const serverEventChan = new MessageChannel()
  const client = new Client({
    server: worker,
    audioPort: audioChan.port1,
    requestPort: clientEventChan.port1,
    listenPort: serverEventChan.port1
  })
  worker.postMessage(
    {
      audioPort: audioChan.port2,
      responsePort: clientEventChan.port2,
      emitPort: serverEventChan.port2
    },
    [audioChan.port2, clientEventChan.port2, serverEventChan.port2]
  )
  const res = await client.request('ping', { msg: 'ping' })
  if (res.status === Err) {
    throw res.err
  }
  console.log(res.ret)
  return client
}
