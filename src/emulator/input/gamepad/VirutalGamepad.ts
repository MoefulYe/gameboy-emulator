import { Config } from '@/emulator/config'
import { type GameboyLayoutButtons, GameboyLayoutButton, type Callback } from './constants'
import { wait } from '@/utils/timer'
import { throttle } from '@/utils/throttle'

export class VirtualGamepad {
  private buttons: GameboyLayoutButtons = 0

  public constructor(
    config: Config,
    private callback: Callback
  ) {
    window.addEventListener(
      'keydown',
      throttle(async (e) => {
        const btn = VirtualGamepad.fromKey(e.key)
        if (btn === undefined) return
        this.down(btn)
        await wait(200)
        this.up(btn)
      })
    )
  }
  public down(button: GameboyLayoutButton) {
    this.buttons = (this.buttons & ~(1 << button)) | (1 << button)
    this.callback(this.buttons)
  }

  public up(button: GameboyLayoutButton) {
    this.buttons = this.buttons & ~(1 << button)
    this.callback(this.buttons)
  }

  private static fromKey(key: string): GameboyLayoutButton | undefined {
    switch (key) {
      case 'a':
        return GameboyLayoutButton.A
      case 's':
        return GameboyLayoutButton.B
      case 'd':
        return GameboyLayoutButton.Select
      case 'f':
        return GameboyLayoutButton.Start
      case 'ArrowDown':
        return GameboyLayoutButton.Down
      case 'ArrowUp':
        return GameboyLayoutButton.Up
      case 'ArrowLeft':
        return GameboyLayoutButton.Left
      case 'ArrowRight':
        return GameboyLayoutButton.Right
      default:
        return undefined
    }
  }
}
