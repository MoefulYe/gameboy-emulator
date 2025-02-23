import { openDB, type DBSchema, type IDBPDatabase } from 'idb'
import type { Save } from '../constants'

interface EmuSchema extends DBSchema {
  saves: {
    key: number
    value: Save
    indexes: {
      lastAccessed: Date
    }
  }
}

export type EmuDB = IDBPDatabase<EmuSchema>

export const createDB = () =>
  openDB<EmuSchema>('emudb', 4, {
    upgrade(db) {
      const store = db.createObjectStore('saves', {
        keyPath: 'id',
        autoIncrement: true
      })
      store.createIndex('lastAccessed', ['metadata', 'lastAccessed'])
    }
  })
