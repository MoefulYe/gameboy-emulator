import { Client } from './client'
import { type InjectionKey, inject } from 'vue'
import Server from './worker/worker?worker'
import { Err } from './constants'
import { useConfig } from './config'
import { useIndexedDB } from './persistance/indexeddb'
import { GameboyLayoutButton as EmulatorButton } from './input/gamepad'

export const emuKey = Symbol() as InjectionKey<Client>
export const useEmulator = () => inject(emuKey)!
export { EmulatorButton }

export const createEmulator = async () => {
  const config = useConfig()
  const worker = new Server()
  const audioChan = new MessageChannel()
  const clientEventChan = new MessageChannel()
  const serverEventChan = new MessageChannel()
  const db = await useIndexedDB()
  const client = new Client({
    db,
    config,
    server: worker,
    audioPort: audioChan.port1,
    requestPort: clientEventChan.port1,
    listenPort: serverEventChan.port1
  })
  client.on('hello', () => console.log('hello'))
  worker.postMessage(
    {
      audioPort: audioChan.port2,
      responsePort: clientEventChan.port2,
      emitPort: serverEventChan.port2
    },
    [audioChan.port2, clientEventChan.port2, serverEventChan.port2]
  )
  const res = await client.ping('ping')
  if (res.status === Err) {
    throw res.err
  }
  console.log(res)
  return client
}
