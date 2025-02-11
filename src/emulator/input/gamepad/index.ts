import { EmulatorButton as GameboyLayoutButton } from 'emulator/pkg/emulator'
import type { Config } from '../../config'
import { PhysicalGamepad } from './PhysicalGamepad'
import { VirtualGamepad } from './VirutalGamepad'
import type { ShallowRef } from 'vue'
export { GameboyLayoutButton }
export type GameboyLayoutButtons = {
  [Button in GameboyLayoutButton]: boolean
}
export type Callback = (buttons: Readonly<GameboyLayoutButtons>) => void
export const enum GamepadMode {
  Virtual,
  Physical
}
export const INPUT_MODE_STR = ['virtual', 'physical'] as const satisfies Record<GamepadMode, string>
export const DEFAULT_GAMEPAD_MODE = GamepadMode.Physical

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
