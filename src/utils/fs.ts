export const openRom = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.click()
  return new Promise<Uint8Array>((r) => {
    input.onchange = async () => {
      const [f] = input.files!
      const buffer = await f.arrayBuffer()
      const u8s = new Uint8Array(buffer)
      r(u8s)
    }
  })
}
