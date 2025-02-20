import { shallowRef, type ShallowRef } from 'vue'
import { GameboyLayoutButton, type GameboyLayoutButtons, type Callback } from './constants'
import { Config } from '@/emulator/config'
import { every } from '@/utils/timer'
import logger from '@/emulator/logger'
import { LogLevel } from '@/emulator/constants'
// https://w3c.github.io/gamepad/#remapping
export type StandardButton =
  | 0
  | 1
  | 2
  | 3
  | 4
  | 5
  | 6
  | 7
  | 8
  | 9
  | 10
  | 11
  | 12
  | 13
  | 14
  | 15
  | 16

export type GamepadMapping = {
  readonly [Button in StandardButton]: GameboyLayoutButton
}

export const DEFAULT_BUTTON_MAPPINGS = [
  GameboyLayoutButton.B, // 0
  GameboyLayoutButton.A, // 1
  GameboyLayoutButton.None, // 2
  GameboyLayoutButton.None, // 3
  GameboyLayoutButton.None, // 4
  GameboyLayoutButton.None, // 5
  GameboyLayoutButton.None, // 6
  GameboyLayoutButton.None, // 7
  GameboyLayoutButton.Select, // 8
  GameboyLayoutButton.Start, // 9
  GameboyLayoutButton.None, // 10
  GameboyLayoutButton.None, // 11
  GameboyLayoutButton.Up, // 12
  GameboyLayoutButton.Down, // 13
  GameboyLayoutButton.Left, // 14
  GameboyLayoutButton.Right, // 15
  GameboyLayoutButton.None // 16
] as const satisfies GamepadMapping

export class PhysicalGamepad {
  private static readonly POLL_INTERVAL = 1000 / 60
  private buttons: GameboyLayoutButtons = 0
  private gamepad?: Gamepad
  public readonly mapping: ShallowRef<GamepadMapping>
  public readonly gamepadId = shallowRef('none')
  private connectListener(e: GamepadEvent) {
    const gamepad = e.gamepad
    const msg = 'connect to gamepad'
    logger(LogLevel.Info, msg)
    this.gamepad = gamepad
    this.gamepadId.value = gamepad.id
  }
  private disconnectListener(e: GamepadEvent) {
    const msg = 'disconnect to gamepad'
    logger(LogLevel.Info, msg)
    this.gamepad = undefined
    this.gamepadId.value = 'none'
  }

  private newButtons(): GameboyLayoutButtons {
    const mapping = this.mapping.value
    let btns = 0
    if (this.gamepad === undefined || !this.gamepad.connected) {
      return btns
    }
    const buttons = this.gamepad.buttons
    for (let i = 0; i < 17; i++) {
      const pos = mapping[i as StandardButton]
      if (pos === GameboyLayoutButton.None) {
        continue
      }
      const pressed = buttons[i].pressed ? 1 : 0
      btns = (btns & ~(1 << pos)) | (pressed << pos)
    }
    return btns
  }

  private hasChanged(newButtons: GameboyLayoutButtons): boolean {
    return (newButtons ^ this.buttons) !== 0
  }

  private poll() {
    const newButtons = this.newButtons()
    const hasChanged = this.hasChanged(newButtons)
    if (hasChanged) {
      this.buttons = newButtons
      this.callback(newButtons)
    }
  }

  constructor(
    config: Config,
    private callback: Callback
  ) {
    window.addEventListener('gamepadconnected', (e) => this.connectListener(e))
    window.addEventListener('gamepaddisconnected', (e) => this.disconnectListener(e))
    this.mapping = config.gamepadMapping
    every(() => this.poll(), PhysicalGamepad.POLL_INTERVAL)
  }
}
