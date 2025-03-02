export class AudioSender {
  constructor(private port: MessagePort) {}
  send(left: Float32Array, right: Float32Array) {
    this.port.postMessage({ left, right }, [left.buffer, right.buffer])
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
