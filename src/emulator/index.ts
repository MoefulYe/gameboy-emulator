import { Client } from './worker/client'
import { type InjectionKey, inject } from 'vue'
import Server from './worker/worker?worker'
import { Err, LogLevel } from './constants'
import { useConfig } from './config'
import { createDB } from './persistance/db'
import log from './logger'

export const emuKey = Symbol() as InjectionKey<Client>
export const useEmulator = () => inject(emuKey)!

export const createEmulator = async () => {
  log(LogLevel.Info, 'initializing...')
  const config = useConfig()
  const worker = new Server()
  const audioChan = new MessageChannel()
  const clientEventChan = new MessageChannel()
  const serverEventChan = new MessageChannel()
  const db = await createDB()
  const client = new Client({
    db,
    config,
    server: worker,
    audioPort: audioChan.port1,
    requestPort: clientEventChan.port1,
    listenPort: serverEventChan.port1
  })
  worker.postMessage(
    {
      audioPort: audioChan.port2,
      responsePort: clientEventChan.port2,
      emitPort: serverEventChan.port2,
      freqScale: config.freqScale.value
    },
    [audioChan.port2, clientEventChan.port2, serverEventChan.port2]
  )
  const res = await client.ping('ping')
  if (res.status === Err) {
    log(LogLevel.Error, 'initialization failed!')
    throw res.err
  }
  log(LogLevel.Info, res.ret)
  return client
}
