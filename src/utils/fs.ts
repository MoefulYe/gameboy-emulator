import { withResolver } from '@/utils/promise'

export const openFile = () => {
  const input = document.createElement('input')
  input.type = 'file'
  const [waiter, resolver] = withResolver<File>()
  input.onchange = () => {
    const [f] = input.files!
    resolver(f)
  }
  input.click()
  return waiter
}
