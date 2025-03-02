import { Status } from '@/utils/event/client_side_event'

/*
shutdown -- 开机 --> running
running -- 暂停 --> paused
running -- 出错 --> aborted
running -- 关机 --> shutdown
paused -- 继续 --> running
paused -- 出错 --> aborted
paused -- 关机 --> shutdown
aborted -- 重置 --> shutdown
aborted -- 关机 --> shutdown
*/
export const enum State {
  // 关机状态，模拟器未启动
  Shutdown,
  // 正常运行状态
  Running,
  // 暂停正常模拟器执行, 但是可以进行单步调试
  Paused,
  // 模拟器出现错误，进入锁定状态，此时只能重启模拟器
  Aborted
}

export const STATE_STR = ['shutdown', 'running', 'paused', 'aborted'] as const satisfies Record<
  State,
  string
>

export const enum LogLevel {
  Off = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Debug = 4
}

export const LOG_STR = ['Off', 'Error', 'Warn', 'Info', 'Debug'] as const satisfies Record<
  LogLevel,
  string
>

export const BASE_FREQ_HZ = 4_194_304
export const VISUAL_FREQ_HZ = 59.7
export const MS_PER_FRAME = 1000 / VISUAL_FREQ_HZ
export const CYCLES_PER_FRAME = BASE_FREQ_HZ / VISUAL_FREQ_HZ
export const DEFAULT_VOLUME = 50

export const Ok = Status.Ok
export const Err = Status.Err

export const TILE_BITMAP_WIDTH = 128
export const TILE_BITMAP_HEIGHT = 192
export const BASE_AUDIO_SAMPLE_RATE = 48000

export type SaveMetadata = {
  readonly createdAt?: Date
  readonly lastAccessed?: Date
  readonly cartTitle: string
}

export type Save = {
  readonly data: Uint8Array
  readonly state: State
  readonly metadata: SaveMetadata
  readonly id: number
}

export const enum SaveMode {
  Create,
  Overwrite
}
