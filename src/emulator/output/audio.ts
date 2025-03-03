export class AudioSender {
  constructor(private port: MessagePort) {}
  send(left: Float32Array, right: Float32Array) {
    const l = new Float32Array(left)
    const r = new Float32Array(right)
    this.port.postMessage({ left: l, right: r }, [l.buffer, r.buffer])
  }
}

export class AudioReceiver {
  constructor(private port: MessagePort) {}
  recv(cb: (left: Float32Array, right: Float32Array) => void) {
    this.port.onmessage = (e) => {
      const { left, right } = e.data
      cb(left, right)
    }
  }
}
