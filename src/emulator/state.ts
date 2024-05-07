export const enum StateType {
  Running = 0,
  Paused = 1,
  Stopped = 2
}

export type ResumeSignal = () => void
export type PauseWait = Promise<void>

export type EmulatorState =
  | { state: StateType.Running }
  | {
      state: StateType.Paused
      wait: PauseWait
      resume: ResumeSignal
    }
  | { state: StateType.Stopped }
