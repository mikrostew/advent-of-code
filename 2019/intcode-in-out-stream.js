const stream = require('stream');

// need in-memory streams to send data to the robot, and get data back
// (see https://nodejs.org/api/stream.html#stream_implementing_a_duplex_stream)
module.exports = class IntcodeInOutStream extends stream.Duplex {
  constructor(initialInputs) {
    // I could optionally input some config here, and pass to the superclass, but whatever
    super();
    // passing no initial input is fine
    this.inputs = initialInputs !== undefined ? initialInputs : [];
  }

  // for reading, just push an input
  // (don't send null for this one, because more data may be coming, even if there are currently no inputs)
  _read(size) {
    if (this.inputs.length != 0) {
      // remove the first input, and push that on the next event loop
      let input = this.inputs[0];
      this.inputs = this.inputs.slice(1);
      // have to send with a newline
      setImmediate(() => this.push(`${input}\n`));
    }
  }

  // when this gets input data, go ahead and push it
  _write(chunk, enc, callback) {
    //console.log(`got output chunk: '${chunk.toString()}'`);
    setImmediate(() => this.push(`${Number(chunk.toString())}\n`));
    //console.log(`current outputs: ${this.outputs}`);
    callback();
  }
}
