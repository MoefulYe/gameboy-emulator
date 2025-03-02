import { Listener, type EventCallback } from '@/utils/event/server_side_event'
import { Requester, type ReqArgs } from '@/utils/event/client_side_event'
import type { ClientSideEvent, ServerSideEvent } from './event'
import AUDIO_WORKER_URL from './audio.worker?url'
import { Stat, useStat } from '../stat'
import { Err, LogLevel, Ok, SaveMode, type Save, type SaveMetadata } from '../constants'
import type { EmuDB } from '../persistance/db'
import type { Config } from '../config'
import { EmuGamepad, useGamepad } from '../input/gamepad'
import type { GameboyLayoutButtons } from '../input/gamepad/constants'
import { onMounted, watch, type ShallowRef } from 'vue'
import log from '../logger'
import { debounce } from '@/utils/debounce'

type CreateOption = {
  config: Config
  db: EmuDB
  listenPort: MessagePort
  requestPort: MessagePort
  audioPort: MessagePort
  server: Worker
}

export class Client {
  private readonly requester: Requester<ClientSideEvent>
  private readonly listener: Listener<ServerSideEvent>
  private readonly server: Worker
  private readonly audioWorkletNode: AudioWorkletNode
  public readonly db: EmuDB
  public readonly config: Config
  public readonly stat: Stat
  public readonly gamepad: EmuGamepad
  private saveId?: number = undefined
  private screenEl: HTMLCanvasElement | null = null
  private audioCtx: AudioContext

  constructor({
    listenPort,
    requestPort,
    audioPort,
    server,
    config,
    db,
    audioCtx,
    audioWorkletNode
  }: CreateOption & {
    audioCtx: AudioContext
    audioWorkletNode: AudioWorkletNode
  }) {
    this.config = config
    this.db = db
    this.requester = new Requester(requestPort)
    this.listener = new Listener(listenPort)
    this.server = server
    this.gamepad = useGamepad(config, (btns) => this.btnAction(btns))
    this.stat = useStat(config)
    this.audioCtx = audioCtx
    this.audioWorkletNode = audioWorkletNode
    audioWorkletNode.port.postMessage({ port: audioPort }, [audioPort])
    this.init()
  }

  static async create(option: CreateOption) {
    const audioCtx = new AudioContext()
    const audioWorkletNode = await createAudioWorker(audioCtx)
    return new Client({ ...option, audioCtx, audioWorkletNode })
  }

  private init() {
    const { cpu, cycles, state, serialBytes: bytes, rom } = this.stat
    const { freqScale } = this.config
    this.on('log', ({ level, msg }) => log(level, msg))
    this.on('update', ({ state: $state, cycles: $cycles, cpu: $cpu, byte: $byte, rom: $rom }) => {
      if ($cycles !== undefined) {
        cycles.value = $cycles
      }
      if ($state !== undefined) {
        state.value = $state
      }
      if ($byte !== undefined) {
        bytes.push($byte)
      }
      if ($cpu !== undefined) {
        cpu.value = $cpu
      }
      if ($rom !== undefined) {
        rom.value = $rom
      }
    })
    watch(
      freqScale,
      debounce((scale: number) => this.requester.request('set-fscale', scale))
    )
  }

  private request<Event extends keyof ClientSideEvent>(
    type: Event,
    data: ReqArgs<ClientSideEvent, Event>,
    transfer: Transferable[] = []
  ) {
    return this.requester.request(type, data, transfer)
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
    const res = await this.request('ping', { msg })
    return res
  }

  public async openRom(rom: Uint8Array) {
    const buf = rom.buffer
    const args = { rom }
    const transfer = [buf]
    const res = await this.request('load-rom', args, transfer)
    if (res.status === Ok) {
      const info = res.ret
      log(LogLevel.Info, `insert rom \`${info.title}\``)
      this.stat.rom.value = info
      this.stat.saveMetaData.value = {
        cartTitle: info.title,
        createdAt: new Date()
      }
    } else {
      const msg = res.err
      log(LogLevel.Error, msg)
    }
  }

  public async useCanvas(elRef: Readonly<ShallowRef<HTMLCanvasElement | null>>) {
    onMounted(async () => {
      const el = elRef.value
      if (el === null) {
        return
      }
      this.screenEl = el
      const canvas = el.transferControlToOffscreen()
      const res = await this.request('set-canvas', { canvas }, [canvas])
      if (res.status === Err) {
        const msg = res.err
        log(LogLevel.Error, msg)
        return
      }
    })
  }

  public async useTilesCanvas(elRef: Readonly<ShallowRef<HTMLCanvasElement | null>>) {
    onMounted(async () => {
      const el = elRef.value
      if (el === null) {
        return
      }
      const canvas = el.transferControlToOffscreen()
      canvas.width = 128
      canvas.height = 192
      const res = await this.request('tile-canvas', { canvas }, [canvas])
      if (res.status === Err) {
        const msg = res.err
        log(LogLevel.Error, msg)
        return
      }
    })
  }

  public btnAction(buttons: GameboyLayoutButtons) {
    return this.request('btn-action', buttons)
  }

  public start() {
    this.request('start', {})
  }

  public pause() {
    this.request('pause', {})
  }

  public step() {
    this.request('step', {})
  }

  public shutdown() {
    this.request('shutdown', {})
  }

  public fullscreen() {
    this.screenEl?.requestFullscreen()
  }

  public async save(mode: SaveMode) {
    const metadata = this.stat.saveMetaData.value
    if (metadata === undefined) {
      log(LogLevel.Warn, 'no cart')
      return
    }
    const res = await this.request('save', {})
    if (res.status === Err) {
      return
    }
    const { data, state } = res.ret
    const metadata1: SaveMetadata = {
      ...metadata,
      lastAccessed: new Date()
    }
    const save =
      mode === SaveMode.Create || this.saveId === undefined
        ? {
            data,
            state,
            metadata: metadata1
          }
        : {
            data,
            state,
            metadata: metadata1,
            id: this.saveId
          }
    this.saveId = await this.db.put('saves', save as any)
    this.stat.saveMetaData.value = metadata1
  }

  public async load(save: Save) {
    const { data, state, metadata, id } = save
    const res = await this.request(
      'load',
      {
        data,
        state
      },
      [data.buffer]
    )
    if (res.status === Err) {
      return
    }
    this.saveId = id
    this.stat.saveMetaData.value = metadata
  }

  public setVolume(volume: number) {
    this.request('set-volume', volume)
  }
}

const createAudioWorker = async (ctx: AudioContext) => {
  await ctx.audioWorklet.addModule(AUDIO_WORKER_URL)
  return new AudioWorkletNode(ctx, 'audio-processor')
}
