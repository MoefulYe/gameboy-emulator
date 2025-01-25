export type Message = {
  Req: Object
  Resp: Object
}

export class WorkerWarpper {
  worker: Worker
  msg_seq = 0
  wakers = new Map<number, (resp: Object) => void>()
  public constructor(url: URL) {
    this.worker = new Worker(url)
    this.worker.onmessage = (e) => {
      const { seq, ...data } = e.data as Message['Resp'] & { seq: number }
      const waker = this.wakers.get(seq)!
      this.wakers.delete(seq)
      waker(data)
    }
  }
  public async request<Msg extends Message>(
    data: Msg['Req'],
    transfer: Transferable[] = []
  ): Promise<Msg['Resp']> {
    const seq = this.msg_seq++
    const payload = {
      ...data,
      seq
    }
    this.worker.postMessage(payload, transfer)
    const waker = await new Promise((resolve) => this.wakers.set(seq, resolve))
    return waker as Msg['Resp']
  }
  public inner() {
    return this.worker
  }
}

export const emuWorker = new WorkerWarpper(new URL('./emu.worker', import.meta.url))
