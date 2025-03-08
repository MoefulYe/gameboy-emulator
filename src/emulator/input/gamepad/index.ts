import type { Config } from '../../config'
import { PhysicalGamepad } from './PhysicalGamepad'
import { VirtualGamepad } from './VirutalGamepad'
import { shallowRef, type ShallowRef } from 'vue'
import { type Callback, GameboyLayoutButton, GamepadMode } from './constants'

export class EmuGamepad {
  public readonly mode: ShallowRef<GamepadMode>
  public readonly virtual: VirtualGamepad
  public readonly physical: PhysicalGamepad
  public readonly activeA = shallowRef(false)
  public readonly activeB = shallowRef(false)
  public constructor(config: Config, callback: Callback) {
    const mode = config.gamepadMode
    this.mode = mode
    this.physical = new PhysicalGamepad(config, (buttons) => {
      if (mode.value === GamepadMode.Physical) {
        callback(buttons)
        this.activeA.value = (buttons & (1 << GameboyLayoutButton.A)) != 0b0000_0000
        this.activeB.value = (buttons & (1 << GameboyLayoutButton.B)) != 0b0000_0000
      }
    })
    this.virtual = new VirtualGamepad(config, (buttons) => {
      if (mode.value === GamepadMode.Virtual) {
        callback(buttons)
        this.activeA.value = (buttons & (1 << GameboyLayoutButton.A)) != 0b0000_0000
        this.activeB.value = (buttons & (1 << GameboyLayoutButton.B)) != 0b0000_0000
      }
    })
  }
}

export const useGamepad = (config: Config, callback: Callback) => new EmuGamepad(config, callback)
