#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const readline = require('readline');
const stream = require('stream');

const INPUT_FILE = './day-7-input.txt';

// enum of opcode names
const OPCODES = {
  HALT: 'halt',
  ADD: 'add',
  MULT: 'mult',
  INPUT: 'input',
  OUTPUT: 'output',
  JNZ: 'jump-if-not-zero',
  JZ: 'jump-if-zero',
  LT: 'less-than',
  EQ: 'equals',
}

// enum of parameter modes
const PARAM_MODES = {
  POS: 'position', // treat parameter as a position in the intcodes
  IMM: 'immediate', // treat parameter as an immediate value
}

// map of opcode values to info
const OPCODE_INFO= {
  1: { code: OPCODES.ADD, length: 4 },
  2: { code: OPCODES.MULT, length: 4 },
  3: { code: OPCODES.INPUT, length: 2 },
  4: { code: OPCODES.OUTPUT, length: 2 },
  5: { code: OPCODES.JNZ, length: 3 },
  6: { code: OPCODES.JZ, length: 3 },
  7: { code: OPCODES.LT, length: 4 },
  8: { code: OPCODES.EQ, length: 4 },
  99: { code: OPCODES.HALT, length: 1 },
}

class Instruction {
  // parse out the current instruction from the list of intcodes at the given position
  constructor(intcodes, position) {
    // (this may be slow, but should be good enough for now)
    // the first value is a combination of parameter modes and opcode
    // convert that value to a string, then split that to get the parts
    let instruction = intcodes[position].toString();
    let opcode = Number(instruction.substring(instruction.length - 2));
    // reverse this for easier access to the mode for each param
    let parameter_modes = instruction.substring(0, instruction.length - 2).split('').reverse().join('');

    // setup the info for the specific opcode
    if (OPCODE_INFO[opcode] === undefined) {
      console.error(`Error: unknown opcode '${opcode}' at position ${position}, from instruction '${instruction}'`);
      process.exit(1);
    } else {
      this.opcode = OPCODE_INFO[opcode].code;
      this.length = OPCODE_INFO[opcode].length;
      this.modes = parameter_modes;
      // also get the parameters to this instruction
      this.params = intcodes.slice(position + 1, position + this.length);
      // and the modes of those params
      this.modes = [];
      for (let i = 0; i < this.params.length; i++) {
        // assume anything that is not 1 is a zero
        this.modes.push( parameter_modes.charAt(i) === '1' ? PARAM_MODES.IMM : PARAM_MODES.POS );
      }
      // console.log(`instr: ${instruction} ${this.params}`);
      // console.log(`opcode: ${this.opcode}`);
      // console.log(`modes: ${this.modes}`);
      // console.log(`length: ${this.length}`);
      // console.log(`params: ${this.params}`);
    }
  }

  isHalt() {
    return this.opcode == OPCODES.HALT;
  }

  // get param based on it's mode
  getParam(number, intcodes) {
    if (this.modes[number] === PARAM_MODES.IMM) {
      // just return the immediate value
      return this.params[number];
    } else {
      // else get it from a position in the code
      return intcodes[this.params[number]];
    }
  }

  // read a line of input
  async readInputLine(rl) {
    return new Promise(resolve => {
      rl.on('line', (input) => {
        //console.log(`Received: ${input}`);
        resolve(input);
      });
    });

  }

  // execute the instruction against the input intcodes, returning the new intcodes
  // (have to pass in & return b/c of immutability in JS)
  async execute(intcodes, currentIP, inputStream, outputStream) {
    // going to update the IP based on the opcode
    let newIP;

    // figure out what to do based on the opcode
    switch (this.opcode) {
      case OPCODES.HALT:
        // this should never be executed, so maybe I should error here, but whatever
        break;

      case OPCODES.ADD:
        // add param1 + param2, storing at param3
        // console.log(`pos ${this.params[2]} = ${this.getParam(0, intcodes)} + ${this.getParam(1, intcodes)}`);
        intcodes[this.params[2]] = this.getParam(0, intcodes) + this.getParam(1, intcodes);
        newIP = currentIP + this.length;
        break;

      case OPCODES.MULT:
        // multiply param1 * param2, storing at param3
        // console.log(`pos ${this.params[2]} = ${this.getParam(0, intcodes)} * ${this.getParam(1, intcodes)}`);
        intcodes[this.params[2]] = this.getParam(0, intcodes) * this.getParam(1, intcodes);
        newIP = currentIP + this.length;
        break;

      case OPCODES.INPUT:
        // take an input value, storing at param1 (assuming POS mode)

        const rl = readline.createInterface({input: inputStream, output: outputStream});

        let input_value = Number(await this.readInputLine(rl));
        // have to remember to close this or the program will hang
        rl.close();

        //console.log(`pos ${this.params[0]} = (input) '${input_value}'`);
        intcodes[this.params[0]] = input_value;
        newIP = currentIP + this.length;
        break;

      case OPCODES.OUTPUT:
        // output a value
        let output_value = this.getParam(0, intcodes); // can be immediate or position for this

        // add a newline because this does not
        outputStream.write(`${output_value}\n`);

        //console.log(`output: ${output_value}`);
        newIP = currentIP + this.length;
        break;

      case OPCODES.JNZ:
        // if param1 != zero, set the IP to param2, else do nothing
        if (this.getParam(0, intcodes) != 0) {
          newIP = this.getParam(1, intcodes);
        } else {
          newIP = currentIP + this.length;
        }
        break;

      case OPCODES.JZ:
        // if param1 == zero, set the IP to param2, else do nothing
        if (this.getParam(0, intcodes) == 0) {
          newIP = this.getParam(1, intcodes);
        } else {
          newIP = currentIP + this.length;
        }
        break;

      case OPCODES.LT:
        // if param1 < param2, store 1 in param3, else store 0
        if (this.getParam(0, intcodes) < this.getParam(1, intcodes)) {
          intcodes[this.params[2]] = 1;
        } else {
          intcodes[this.params[2]] = 0;
        }
        newIP = currentIP + this.length;
        break;

      case OPCODES.EQ:
        // if param1 == param2, store 1 in param3, else store 0
        if (this.getParam(0, intcodes) == this.getParam(1, intcodes)) {
          intcodes[this.params[2]] = 1;
        } else {
          intcodes[this.params[2]] = 0;
        }
        newIP = currentIP + this.length;
        break;
    }

    // return the modified program and instruction pointer
    return { intcodes, newIP };
  }
}


// program that uses streams to read input and write output
class IntcodeProgram {
  constructor(intcodesStr, inputStream, outputStream) {
    // split on comma and convert to ints
    this.intcodes = intcodesStr.split(',').map(Number);
    this.instruction_pointer = 0;
    this.instruction = undefined;
    this.inputStream = inputStream;
    this.outputStream = outputStream;
  }

  static fromFile(file, inputStream, outputStream) {
    // read file as string
    let intcodesStr = fs.readFileSync(file, "utf-8");
    return new IntcodeProgram(intcodesStr, inputStream, outputStream);
  }

  async run() {
    // main program loop
    while(true) {
      this.instruction = this.parseCurrentInstruction();
      // check for halt
      if (this.instruction.isHalt()) { break; }
      let result = await this.instruction.execute(this.intcodes, this.instruction_pointer, this.inputStream, this.outputStream);
      this.intcodes = result.intcodes;
      this.instruction_pointer = result.newIP;
    }
  }

  parseCurrentInstruction() {
    return new Instruction(this.intcodes, this.instruction_pointer);
  }
}

class AmplifierInputStream extends stream.Readable {
  // https://nodejs.org/api/stream.html#stream_class_stream_readable
  constructor(phase, inputValue) {
    super();
    this.inputs = [ `${phase}\n`, `${inputValue}\n` ];
  }

  // have to implement this function
  _read(size) {
    // no more data
    if (this.inputs.length == 0) {
      setImmediate(() => this.push(null));
    } else {
      // remove the first input, and push that on the next event loop
      let input = this.inputs[0];
      this.inputs = this.inputs.slice(1);
      setImmediate(() => this.push(input));
    }
  }
}

// Writable stream that parses Intcode output to Numbers
// (https://stackoverflow.com/a/21583831)
class IntcodeOutputStream extends stream.Writable {
  // https://nodejs.org/api/stream.html#stream_class_stream_writable
  constructor() {
    super();
    this.outputs = [];
  }

  // override _write()
  _write(chunk, enc, next) {
    //console.log(`got output chunk: '${chunk.toString()}'`);
    this.outputs.push(Number(chunk.toString()));
    //console.log(`current outputs: ${this.outputs}`);
    next();
  }

  getOutputs() {
    return this.outputs;
  }
}

class AmplifierChain {
  constructor(intcodesStr) {
    this.intcodesStr = intcodesStr;
  }

  static fromFile(file) {
    // do it here, so we only read the file once
    let intcodesStr = fs.readFileSync(file, "utf-8");
    return new AmplifierChain(intcodesStr);
  }

  async runAmp(phase, inputValue) {
    // setup input and output streams for this amp
    let ampInput = new AmplifierInputStream(phase, inputValue);
    let ampOutput = new IntcodeOutputStream();

    // run the amp
    let ampProgram = new IntcodeProgram(this.intcodesStr, ampInput, ampOutput);
    await ampProgram.run();

    // get the output value from the stream, somehow
    let ampOutputs = ampOutput.getOutputs();
    //console.log(ampOutputs);
    return ampOutputs[0];
  }

  // 5 amplifiers, each with a unique phase setting 0-4
  // what is the largest output?
  async findLargestOutput() {
    let largestOutput = 0;
    // try every possible permutation, and find the largest one
    // (5*4*3*2*1 = 120, so not bad at all, even though the nested for loops looks scary)
    const aPhases = [0, 1, 2, 3, 4];

    for (let a = 0; a < aPhases.length; a++) {
      // only use phases that are not being used already
      let bPhases = aPhases.filter((el, i) => i != a);
      for (let b = 0; b < bPhases.length; b++) {
        // only use phases that are not being used already
        let cPhases = bPhases.filter((el, i) => i != b);
        for (let c = 0; c < cPhases.length; c++) {
          // only use phases that are not being used already
          let dPhases = cPhases.filter((el, i) => i != c);
          for (let d = 0; d < dPhases.length; d++) {
            // at this point E can only be one thing, but whatever I like the symmetry here
            let ePhases = dPhases.filter((el, i) => i != d);
            for (let e = 0; e < ePhases.length; e++) {
              // initial input is 0 - then pipe that thru each amp
              let outputValue0 = await this.runAmp(aPhases[a], 0);
              let outputValue1 = await this.runAmp(bPhases[b], outputValue0);
              let outputValue2 = await this.runAmp(cPhases[c], outputValue1);
              let outputValue3 = await this.runAmp(dPhases[d], outputValue2);
              let outputValue4 = await this.runAmp(ePhases[e], outputValue3);

              console.log(`${aPhases[a]}${bPhases[b]}${cPhases[c]}${dPhases[d]}${ePhases[e]} --> ${outputValue4}`);
              if (outputValue4 > largestOutput) {
                largestOutput = outputValue4;
              }
            }
          }
        }
      }
    }
    console.log(`largest output: ${largestOutput}`);
  }
}

// input the program and run it
const chain = AmplifierChain.fromFile(INPUT_FILE);

// some test programs from the description:
//
// max signal should be 139629729 (from sequence 9,8,7,6,5)
// '3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5'
// const chain = new AmplifierChain('3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5');
//
// max signal should be 18216 (from sequence 9,7,8,5,6)
// '3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10'
// const chain = new AmplifierChain('3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10');

chain.findLargestOutput();
