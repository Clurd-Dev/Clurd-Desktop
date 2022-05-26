self.addEventListener('message', async function(e) {
    const buffer = await e.data.arrayBuffer();
    let byteArray = new Int8Array(buffer);
    self.postMessage(byteArray);
    self.close();
  }, false);