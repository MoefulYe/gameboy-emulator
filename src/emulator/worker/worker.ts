import { withResolver } from '@/utils/promise'
import { Server, type CreateOption } from './server'

const [p, r] = withResolver<Server>()

self.onmessage = async (e: MessageEvent<CreateOption>) => {
  const { audioPort, responsePort, emitPort } = e.data
  const server = await Server.create({
    audioPort,
    responsePort,
    emitPort
  })
  r(server)
}

const server = await p
