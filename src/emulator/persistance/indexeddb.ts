import { openDB, type IDBPDatabase } from 'idb'

export const useIndexedDB = async () => {
  const db = await openDB('emu', 4, {})
  return db
}

export type DB = IDBPDatabase<unknown>
