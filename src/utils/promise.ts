export const withResolver = <T>(): [Promise<T>, (value: T) => void] => {
  let resolver: undefined | ((value: T) => void) = undefined
  const promise = new Promise<T>((r) => (resolver = r))
  return [promise, resolver!]
}
