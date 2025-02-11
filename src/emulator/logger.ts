import { reactive } from 'vue'
import { LogLevel } from './constants'
import { useLocalStorage, type LocalStorageKey } from './persistance/localstorage'
import { every } from '@/utils/timer'

export type Log = {
  level: LogLevel
  msg: string
  id: number
}

let id = 0
const FILTER_KEY = 'logger-filter' as LocalStorageKey<LogLevel>
export const logs = reactive<Readonly<Log>[]>([])
export const filter = useLocalStorage(FILTER_KEY, LogLevel.Info)
const log = (level: LogLevel, msg: string) => {
  if (filter.value < level) return
  logs.push({ id, level, msg })
  id++
}
export default log
