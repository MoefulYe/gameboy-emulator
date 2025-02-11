import logger from './logger'
import { Listener, type EventCallback } from '@/utils/event/server_side_event'
import { Requester, type ReqArgs } from '@/utils/event/client_side_event'
import { AudioReceiver } from './output/audio'
import type { ClientSideEvent, ServerSideEvent } from './worker/event'
import { useStat } from './stat'
import { Err } from './constants'
import type { DB } from './persistance/indexeddb'
import type { Config } from './config'
import { EmuGamepad, useGamepad, type GameboyLayoutButtons } from './input/gamepad'
import { onMounted, type ShallowRef } from 'vue'

type CreateOption = {
  config: Config
  db: DB
  listenPort: MessagePort
  requestPort: MessagePort
  audioPort: MessagePort
  server: Worker
}

export class Client {
  private readonly requester: Requester<ClientSideEvent>
  private readonly listener: Listener<ServerSideEvent>
  private readonly audioReceiver: AudioReceiver
  private readonly server: Worker
  private readonly db: DB
  public readonly config: Config
  public readonly stat = useStat()
  public readonly gamepad: EmuGamepad

  constructor({ listenPort, requestPort, audioPort, server, config, db }: CreateOption) {
    this.config = config
    this.db = db
    this.requester = new Requester(requestPort)
    this.listener = new Listener(listenPort)
    this.audioReceiver = new AudioReceiver(audioPort)
    this.server = server
    this.gamepad = useGamepad(config, (btns) => this.btnAction(btns))
    this.useLog()
  }

  private request<Event extends keyof ClientSideEvent>(
    type: Event,
    data: ReqArgs<ClientSideEvent, Event>,
    transfer: Transferable[] = []
  ) {
    return this.requester.request(type, data, transfer)
  }

  private useLog() {
    this.use('log', ({ level, msg }) => logger(level, msg))
  }

  private on<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.on(event, callback)
  }

  private off<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.off(event, callback)
  }

  private use<Event extends keyof ServerSideEvent>(
    event: Event,
    callback: EventCallback<ServerSideEvent[Event]>
  ) {
    this.listener.use(event, callback)
  }

  public async ping(msg: string) {
    const res = await this.request('ping', { msg }, [])
    return res
  }

  public async openRom(rom: Uint8Array) {
    const buf = rom.buffer
    const args = { rom }
    const transfer = [buf]
    const res = await this.request('load-rom', args, transfer)
    if (res.status === Err) {
      const msg = res.err
      console.log(msg)
      return
    }
  }

  public async useCanvas(elRef: Readonly<ShallowRef<HTMLCanvasElement | null>>) {
    onMounted(async () => {
      const el = elRef.value
      if (el === null) {
        return
      }
      const canvas = el.transferControlToOffscreen()
      const res = await this.request('set-canvas', { canvas }, [canvas])
      if (res.status === Err) {
        const msg = res.err
        console.log(msg)
        return
      }
    })
  }

  public btnAction(buttons: Readonly<GameboyLayoutButtons>) {
    return this.request('btn-action', buttons)
  }
}
