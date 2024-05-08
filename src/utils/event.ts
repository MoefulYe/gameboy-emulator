import { onMounted, onUnmounted } from 'vue'
export type EventType = string | symbol | number
export type EventListener<Args extends unknown[]> = (...args: Args) => unknown
export type EventListeners<Args extends unknown[]> = Array<EventListener<Args>>
type EventListenerMap<Events extends Record<EventType, unknown[]>> = {
  [Event in keyof Events]: EventListeners<Events[Event]>
}

export interface EventDispatcher<Events extends Record<EventType, unknown[]>> {
  on<Event extends keyof Events>(event: Event, listener: EventListener<Events[Event]>): void
  off<Event extends keyof Events>(event: Event, listener: EventListener<Events[Event]>): void
}

export interface EventEmitter<Events extends Record<EventType, unknown[]>>
  extends EventDispatcher<Events> {
  emit<Event extends keyof Events>(event: Event, ...args: Events[Event]): void
}

class EventEmitterImpl<Events extends Record<EventType, unknown[]>>
  implements EventEmitter<Events>
{
  constructor(private listeners: Partial<EventListenerMap<Events>> = {}) {}
  on<Event extends keyof Events>(event: Event, listener: EventListener<Events[Event]>): void {
    const listeners = this.listeners[event]
    if (listeners !== undefined) {
      listeners.push(listener)
    } else {
      this.listeners[event] = [listener]
    }
  }
  off<Event extends keyof Events>(event: Event, listener: EventListener<Events[Event]>): void {
    const listeners = this.listeners[event]
    if (listeners !== undefined) {
      const index = listeners.indexOf(listener)
      if (index !== -1) {
        listeners.splice(index, 1)
      }
    }
  }
  emit<Event extends keyof Events>(event: Event, ...args: Events[Event]): void {
    const listeners = this.listeners[event]
    if (listeners !== undefined) {
      for (const listener of listeners) {
        listener(...args)
      }
    }
  }
}

export const createEventEmitter = <
  Events extends Record<EventType, unknown[]>
>(): EventEmitter<Events> => new EventEmitterImpl<Events>()

export const useListener = <
  Events extends Record<EventType, unknown[]>,
  Event extends keyof Events
>(
  dispatcher: EventDispatcher<Events>,
  event: Event,
  listener: EventListener<Events[Event]>
) => {
  onMounted(() => dispatcher.on(event, listener))
  onUnmounted(() => dispatcher.off(event, listener))
}
