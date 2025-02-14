type Fn<Args extends unknown[], Ret> = (..._: Args) => Ret

export const throttle = <Args extends unknown[]>(
  fn: Fn<Args, unknown>,
  wait: number = 300
): Fn<Args, void> => {
  let busy = false
  return (...args: Args) => {
    if (busy) {
      return
    }
    busy = true
    setTimeout(() => (busy = false), wait)
    fn(...args)
  }
}
