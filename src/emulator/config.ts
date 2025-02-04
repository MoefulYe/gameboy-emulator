import { useLocalStorage, type LocalStorageKey } from './persistance/localstorage'
import { computed } from 'vue'
import { BASE_FREQ_HZ } from './constants'
import { DEFAULT_BUTTON_MAPPINGS, type GamepadMapping } from './input/gamepad/PhysicalGamepad'
import { DEFAULT_GAMEPAD_MODE, GamepadMode } from './input/gamepad'

const VOLUME = 'volume' as LocalStorageKey<number>
const FREQ_SCALE = 'freq-scale' as LocalStorageKey<number>
const GAMEPAD_MAPPING = 'gamepad-mapping' as LocalStorageKey<GamepadMapping>
const GAMEPAD_MODE = 'gamepad-mode' as LocalStorageKey<GamepadMode>

export class Config {
  public readonly volume = useLocalStorage(VOLUME, 50)
  public readonly freqScale = useLocalStorage(FREQ_SCALE, 1.0)
  public readonly gamepadMapping = useLocalStorage(GAMEPAD_MAPPING, DEFAULT_BUTTON_MAPPINGS)
  public readonly gamepadMode = useLocalStorage(GAMEPAD_MODE, DEFAULT_GAMEPAD_MODE)

  public readonly freqHz = computed(() => BASE_FREQ_HZ * this.freqScale.value)
}

export const useConfig = () => new Config()
