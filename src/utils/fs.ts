export const openFile = () => {
  const input = document.createElement('input')
  input.type = 'file'
  input.click()
  return new Promise<File>((r) => {
    input.onchange = async () => {
      const [f] = input.files!
      r(f)
    }
  })
}
