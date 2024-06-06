export class ExclusiveError extends Error {
  constructor() {
    super('Exclusive Error')
  }
}

export const exclusive = <Args extends unknown[], Ret>(
  target: (...args: Args) => Ret,
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  _: unknown
) => {
  let running = false
  return (...args: Args): Ret => {
    if (running) {
      throw new ExclusiveError()
    }
    running = true
    try {
      return target(...args)
    } finally {
      running = false
    }
  }
}

export class DebounceError extends Error {
  constructor() {
    super('Debounce Error')
  }
}

export const debounce =
  <Args extends unknown[], Ret>(ms: number = 300) =>
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  (target: (...args: Args) => Ret, _: unknown) => {
    let allow = true
    return (...args: Args): Ret => {
      if (!allow) {
        throw new DebounceError()
      }
      allow = false
      setTimeout(() => {
        allow = true
      }, ms)
      return target(...args)
    }
  }
