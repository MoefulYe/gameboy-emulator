import type { Config } from '../../config'
import { PhysicalGamepad } from './PhysicalGamepad'
import { VirtualGamepad } from './VirutalGamepad'
import type { ShallowRef } from 'vue'
import { type Callback, GamepadMode } from './constants'

export class EmuGamepad {
  public readonly mode: ShallowRef<GamepadMode>
  public readonly virtual: VirtualGamepad
  public readonly physical: PhysicalGamepad
  public constructor(config: Config, callback: Callback) {
    const mode = config.gamepadMode
    this.mode = mode
    this.physical = new PhysicalGamepad(config, (buttons) => {
      if (mode.value === GamepadMode.Physical) {
        callback(buttons)
      }
    })
    this.virtual = new VirtualGamepad(config, (buttons) => {
      if (mode.value === GamepadMode.Virtual) {
        callback(buttons)
      }
    })
  }
}

export const useGamepad = (config: Config, callback: Callback) => new EmuGamepad(config, callback)
