import { Config } from '@/emulator/config'
import type { GameboyLayoutButtons, GameboyLayoutButton, Callback } from './constants'

export class VirtualGamepad {
  private buttons: GameboyLayoutButtons = 0

  public constructor(
    config: Config,
    private callback: Callback
  ) {}
  public down(button: GameboyLayoutButton) {
    this.buttons = (this.buttons & ~(1 << button)) | (1 << button)
    this.callback(this.buttons)
  }

  public up(button: GameboyLayoutButton) {
    this.buttons = this.buttons & ~(1 << button)
    this.callback(this.buttons)
  }
}
