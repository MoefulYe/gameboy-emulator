export const wait = (ms: number): Promise<void> =>
  new Promise((resolve) => setTimeout(() => resolve(), ms))

export const every = async (callback: () => void, ms: number) => {
  // eslint-disable-next-line no-constant-condition
  while (true) {
    const ts = performance.now()
    callback()
    const diff = performance.now() - ts
    const delta = ms - diff
    if (delta > 0) {
      await wait(delta)
    } else {
      console.log('every')
    }
  }
}
