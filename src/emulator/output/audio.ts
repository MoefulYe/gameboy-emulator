export class AudioSender {
  constructor(private port: MessagePort) {}
  send(data: Float32Array) {
    const $data = new Float32Array(data)
    this.port.postMessage($data, [$data.buffer])
  }
}

export class AudioReceiver {
  constructor(private port: MessagePort) {}
  recv(cb: (data: Float32Array) => void) {
    this.port.onmessage = (e) => {
      cb(e.data)
    }
  }
}
