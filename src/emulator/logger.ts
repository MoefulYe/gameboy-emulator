import { shallowReactive } from 'vue'
import { LogLevel } from './constants'
import { useLocalStorage, type LocalStorageKey } from '../utils/localstorage'

export type Log = {
  readonly level: LogLevel
  readonly msg: string
  readonly id: number
}

let id = 0
const FILTER_KEY = 'logger-filter' as LocalStorageKey<LogLevel>
export const logs = shallowReactive<Readonly<Log>[]>([])
export const filter = useLocalStorage(FILTER_KEY, LogLevel.Info)
const log = (level: LogLevel, msg: string) => {
  if (filter.value < level) return
  logs.push({ id, level, msg })
  id++
}
export default log
