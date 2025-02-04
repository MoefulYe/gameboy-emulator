import { Config } from '@/emulator/config'
import type { GameboyLayoutButtons, GameboyLayoutButton, Callback } from '.'

export class VirtualGamepad {
  private buttons: GameboyLayoutButtons = [false, false, false, false, false, false, false, false]

  public constructor(
    config: Config,
    private callback: Callback
  ) {}
  public down(button: GameboyLayoutButton) {
    this.buttons[button] = true
    this.callback(this.buttons)
  }

  public up(button: GameboyLayoutButton) {
    this.buttons[button] = false
    this.callback(this.buttons)
  }
}
