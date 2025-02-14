type Fn<Args extends unknown[], Ret> = (..._: Args) => Ret
type Option = {
  immediate: true
}
export const debounce = <Args extends unknown[]>(
  fn: Fn<Args, unknown>,
  delay: number = 300,
  opt?: Option
): Fn<Args, void> => {
  let timer: undefined | number = undefined
  if (opt?.immediate === true) {
    return (...args) => {
      if (timer === undefined) {
        timer = setTimeout(fn, delay, ...args)
      } else {
        clearTimeout(timer)
        timer = setTimeout(fn, delay, ...args)
      }
    }
  } else {
    return (...args) => {
      clearTimeout(timer)
      timer = setTimeout(fn, delay, ...args)
    }
  }
}
