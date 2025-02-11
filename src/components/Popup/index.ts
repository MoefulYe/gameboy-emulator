import { inject, type InjectionKey, type Ref } from 'vue'
import Popup from './Popup.vue'
export default Popup

export const popupShowKey = Symbol() as InjectionKey<Ref<boolean>>
export const useShowPopup = () => inject(popupShowKey)!
