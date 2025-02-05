import { type ShallowRef } from 'vue'
import { GameboyLayoutButton, type GameboyLayoutButtons, type Callback } from '.'
import { Config } from '@/emulator/config'
import { every } from '@/utils/timer'
// https://w3c.github.io/gamepad/#remapping
type StandardButton = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16

export type GamepadMapping = {
  readonly [Button in StandardButton]: GameboyLayoutButton | null
}

export const DEFAULT_BUTTON_MAPPINGS: GamepadMapping = [
  GameboyLayoutButton.B, // 0
  GameboyLayoutButton.A, // 1
  null, // 2
  null, // 3
  null, // 4
  null, // 5
  null, // 6
  null, // 7
  GameboyLayoutButton.Select, // 8
  GameboyLayoutButton.Start, // 9
  null, // 10
  null, // 11
  GameboyLayoutButton.Up, // 12
  GameboyLayoutButton.Down, // 13
  GameboyLayoutButton.Left, // 14
  GameboyLayoutButton.Right, // 15
  null // 16
]

export class PhysicalGamepad {
  private static readonly POLL_INTERVAL = 1000 / 60
  private buttons: Readonly<GameboyLayoutButtons> = [
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false
  ]
  private gamepad?: Gamepad
  private mapping: ShallowRef<GamepadMapping>
  private connectListener(e: GamepadEvent) {
    this.gamepad = e.gamepad
  }
  private disconnectListener() {
    this.gamepad = undefined
  }

  private newButtons(): Readonly<GameboyLayoutButtons> {
    const mapping = this.mapping.value
    const buttons = [false, false, false, false, false, false, false, false]
    if (this.gamepad === undefined || !this.gamepad.connected) {
      return buttons as any
    }
    const _buttons = this.gamepad.buttons
    for (let i = 0; i < 17; i++) {
      const { pressed } = _buttons[i]
      const gbBtn = mapping[i as StandardButton]
      if (gbBtn === null) {
        continue
      }
      buttons[gbBtn] ||= pressed
    }
    return buttons as any
  }

  private hasChanged(oldButtons: Readonly<GameboyLayoutButtons>): boolean {
    for (let i = 0; i < 8; i++) {
      if (this.buttons[i as GameboyLayoutButton] !== oldButtons[i as GameboyLayoutButton])
        return true
    }
    return false
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
    this.gamepad = undefined
    window.addEventListener('gamepadconnected', (e) => this.connectListener(e))
    window.addEventListener('gamepaddisconnected', () => this.disconnectListener())
    this.mapping = config.gamepadMapping
    every(() => this.poll(), PhysicalGamepad.POLL_INTERVAL)
  }
}
