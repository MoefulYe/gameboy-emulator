import { inject, type InjectionKey, type Ref } from 'vue'
import SideBar from './SideBar.vue'
export default SideBar

export const sideBarShowKey = Symbol() as InjectionKey<Ref<boolean>>
export const useShowSideBar = () => inject(sideBarShowKey)!
