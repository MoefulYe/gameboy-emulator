export const enum GameboyLayoutButton {
  Right = 0,
  Left = 1,
  Up = 2,
  Down = 3,
  A = 4,
  B = 5,
  Start = 6,
  Select = 7,
  None = 8
}

export type GameboyLayoutButtons = number
export type Callback = (buttons: GameboyLayoutButtons) => void
export const enum GamepadMode {
  Virtual,
  Physical
}
export const INPUT_MODE_STR = ['virtual', 'physical'] as const satisfies Record<GamepadMode, string>
export const DEFAULT_GAMEPAD_MODE = GamepadMode.Physical
