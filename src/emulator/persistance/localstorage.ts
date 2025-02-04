import { shallowRef, watch, type ShallowRef } from 'vue'
type T1 = number | string | boolean | null
type T2 = readonly T4[]
type T3 = { readonly [key: string]: T4 }
type T4 = T1 | T2 | T3

// eslint-disable-next-line @typescript-eslint/no-unused-vars
interface KeyConstrait<T extends T4> {}
export type LocalStorageKey<T extends T4> = string & KeyConstrait<T>
const getItem = <T extends T4>(key: LocalStorageKey<T>): T | undefined => {
  const val = localStorage.getItem(key)
  if (val === null) {
    return undefined
  } else {
    return JSON.parse(val)
  }
}
const setItem = <T extends T4>(key: LocalStorageKey<T>, val: T) => {
  const serded = JSON.stringify(val)
  localStorage.setItem(key, serded)
}

export function useLocalStorage<T extends T4>(key: LocalStorageKey<T>): ShallowRef<T | undefined>
export function useLocalStorage<T extends T4>(key: LocalStorageKey<T>, defaultVal: T): ShallowRef<T>

export function useLocalStorage<T extends T4>(
  key: LocalStorageKey<T>,
  defaultVal?: T
): ShallowRef<T | undefined> {
  const val = shallowRef<T | undefined>(getItem(key) ?? defaultVal)
  let dirty = false
  watch(val, () => (dirty = true))
  window.addEventListener('unload', () => {
    if (dirty) {
      setItem(key, val.value)
    }
  })
  return val as ShallowRef<T | undefined>
}
