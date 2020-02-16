#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const readline = require('readline');
//const prompt_sync = require('prompt-sync')();

const INPUT_FILE = './day-5-input.txt';

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
    let intcodesStr = fs.readFileSync(INPUT_FILE, "utf-8");
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


class AmplifierChain {
  constructor(intcodesStr) {
    this.intcodesStr = intcodesStr;
  }

  static fromFile(file) {
    // do it here, so we only read the file once
    let intcodesStr = fs.readFileSync(INPUT_FILE, "utf-8");
    return new AmplifierChain(intcodesStr);
  }

  // 5 amplifiers, each with a unique phase setting 0-4
  // what is the largest output?
  findLargestOutput() {
    // TODO - try every possible combination, and find the largest one
    // (5*5*5*5*5 = 3125, which is not that bad)
  }
}


// input the program and run it
const program = IntcodeProgram.fromFile(INPUT_FILE, process.stdin, process.stdout);

// some test programs from the description:
//
// max signal should be 43210 (from sequence 4,3,2,1,0)
// '3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0'
//
// max signal should be 54321 (from sequence 0,1,2,3,4)
// '3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0'
//
// max signal should be 65210 (from sequence 1,0,4,3,2)
// '3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0'

program.run();
