self.onmessage = e => {
    console.log(e.data)
    self.postMessage({
        seq: e.data.seq,
        data: 'hello'
    })
}