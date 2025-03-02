import { withResolver } from '@/utils/promise'
import { Server, type CreateOption } from './server'

const [p, r] = withResolver<Server>()

self.onmessage = async (e: MessageEvent<CreateOption>) => {
  const server = await Server.create(e.data)
  r(server)
}
