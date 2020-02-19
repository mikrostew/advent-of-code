#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const readline = require('readline');
const stream = require('stream');

const INPUT_FILE = './day-9-input.txt';

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
  ADJ: 'adjust-relative-base',
}

// enum of parameter modes
const PARAM_MODES = {
  POS: 'position', // treat parameter as a position in the intcodes
  IMM: 'immediate', // treat parameter as an immediate value
  REL: 'relative', // treat parameter as relative to the current base
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
  9: { code: OPCODES.ADJ, length: 2 },
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
        // TODO: do this in a simpler way?
        switch (parameter_modes.charAt(i)) {
          case '0':
          case '':
            this.modes.push(PARAM_MODES.POS);
            break;
          case '1':
            this.modes.push(PARAM_MODES.IMM);
            break;
          case '2':
            this.modes.push(PARAM_MODES.REL);
            break;
          default:
            console.error(`Unknown parameter mode '${parameter_modes.charAt(i)}', at position ${position}`);
            break
        }
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
  getParam(number, relativeBase, intcodes) {
    if (this.modes[number] === PARAM_MODES.IMM) {
      // just return the immediate value
      return this.params[number];
    } else if (this.modes[number] === PARAM_MODES.REL) {
      // use the relative base to get the value
      return intcodes[relativeBase + this.params[number]];
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
  async execute(intcodes, currentIP, currentBase, inputStream, outputStream) {
    // going to update the IP based on the opcode
    let newIP;
    // update the relative base for that opcode
    let newBase = currentBase;

    // figure out what to do based on the opcode
    switch (this.opcode) {
      case OPCODES.HALT:
        // this should never be executed, so maybe I should error here, but whatever
        break;

      case OPCODES.ADD:
        // add param1 + param2, storing at param3
        // console.log(`pos ${this.params[2]} = ${this.getParam(0, intcodes)} + ${this.getParam(1, intcodes)}`);
        intcodes[this.params[2]] = this.getParam(0, currentBase, intcodes) + this.getParam(1, currentBase, intcodes);
        newIP = currentIP + this.length;
        break;

      case OPCODES.MULT:
        // multiply param1 * param2, storing at param3
        // console.log(`pos ${this.params[2]} = ${this.getParam(0, intcodes)} * ${this.getParam(1, intcodes)}`);
        intcodes[this.params[2]] = this.getParam(0, currentBase, intcodes) * this.getParam(1, currentBase, intcodes);
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
        let output_value = this.getParam(0, currentBase, intcodes); // can be immediate or position for this

        // add a newline because this does not
        outputStream.write(`${output_value}\n`);

        //console.log(`output: ${output_value}`);
        newIP = currentIP + this.length;
        break;

      case OPCODES.JNZ:
        // if param1 != zero, set the IP to param2, else do nothing
        if (this.getParam(0, currentBase, intcodes) != 0) {
          newIP = this.getParam(1, currentBase, intcodes);
        } else {
          newIP = currentIP + this.length;
        }
        break;

      case OPCODES.JZ:
        // if param1 == zero, set the IP to param2, else do nothing
        if (this.getParam(0, currentBase, intcodes) == 0) {
          newIP = this.getParam(1, currentBase, intcodes);
        } else {
          newIP = currentIP + this.length;
        }
        break;

      case OPCODES.LT:
        // if param1 < param2, store 1 in param3, else store 0
        if (this.getParam(0, currentBase, intcodes) < this.getParam(1, currentBase, intcodes)) {
          intcodes[this.params[2]] = 1;
        } else {
          intcodes[this.params[2]] = 0;
        }
        newIP = currentIP + this.length;
        break;

      case OPCODES.EQ:
        // if param1 == param2, store 1 in param3, else store 0
        if (this.getParam(0, currentBase, intcodes) == this.getParam(1, currentBase, intcodes)) {
          intcodes[this.params[2]] = 1;
        } else {
          intcodes[this.params[2]] = 0;
        }
        newIP = currentIP + this.length;
        break;

      case OPCODES.ADJ:
        // adjust relative base by param1
        newBase = currentBase + this.getParam(0, currentBase, intcodes);

        newIP = currentIP + this.length;
        break;
    }

    // return the modified program and instruction pointer
    return { intcodes, newIP, newBase };
  }
}


// program that uses streams to read input and write output
class IntcodeProgram {
  constructor(intcodesStr, inputStream, outputStream) {
    // split on comma and convert to ints
    this.intcodes = intcodesStr.split(',').map(Number);
    this.instruction_pointer = 0;
    this.relative_base = 0;
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
      let result = await this.instruction.execute(this.intcodes, this.instruction_pointer, this.relative_base, this.inputStream, this.outputStream);
      this.intcodes = result.intcodes;
      this.instruction_pointer = result.newIP;
      this.relative_base = result.newBase;
    }
  }

  parseCurrentInstruction() {
    return new Instruction(this.intcodes, this.instruction_pointer);
  }
}

// input the program and run it
//const program = IntcodeProgram.fromFile(INPUT_FILE, process.stdin, process.stdout);

// some test programs from the description:
//
// should produce a copy of itself as output
// TODO: prints a ton of undefined's
const program = new IntcodeProgram('109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99', process.stdin, process.stdout);
//
// should output a 16-digit number - OK
//const program = new IntcodeProgram('1102,34915192,34915192,7,4,7,99,0', process.stdin, process.stdout);
//
// should output the large number in the middle - OK
//const program = new IntcodeProgram('104,1125899906842624,99', process.stdin, process.stdout);

program.run();
