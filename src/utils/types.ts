export type DeepReadonly<T> = T extends {}
  ? {
      readonly [P in keyof T]: DeepReadonly<T[P]>
    }
  : T
