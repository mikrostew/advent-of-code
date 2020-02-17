#!/usr/bin/env node

'use strict';

// this should probably use classes and such to be nicer, but whatever

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-2-input.txt';

// run the input intcode program
function run_program(input_intcodes) {
  let prog_position = 0; // program counter
  let prog_intcodes = input_intcodes;

  // handle the opcode, and exit if we hit the end
  let result = do_opcode(prog_position, prog_intcodes);
  while (result.action != 'halt') {
    // increment the position by 4, since all instructions are the same length
    prog_position += 4;
    prog_intcodes = result.intcodes;

    // and handle the next opcode
    result = do_opcode(prog_position, prog_intcodes);
  }

  // all done
  return prog_intcodes;
}

// handle the opcode at position `pc`, in program `codes`
// and return the result as an object
function do_opcode(pc, codes) {
  let action = 'continue'; // assume we will continue execution
  let pos1, pos2, pos3, val1, val2, result;

  console.log(`[${pc}] ${codes[pc]} ${codes[pc+1]} ${codes[pc+2]} ${codes[pc+3]}`);

  switch (codes[pc]) {
    case 1:
      // add [pc+1] * [pc+2,] storing at [pc+3]
      pos1 = codes[pc+1];
      pos2 = codes[pc+2];
      pos3 = codes[pc+3];
      val1 = codes[pos1];
      val2 = codes[pos2];
      result = val1 + val2;
      codes[pos3] = result;

      console.log(`[${pc}] 1: add ${pos1}:${val1} + ${pos2}:${val2}, store ${result} at pos ${pos3}`);
      break;
    case 2:
      // multiply [pc+1] * [pc+2,] storing at [pc+3]
      pos1 = codes[pc+1];
      pos2 = codes[pc+2];
      pos3 = codes[pc+3];
      val1 = codes[pos1];
      val2 = codes[pos2];
      result = val1 * val2;
      codes[pos3] = result;

      console.log(`[${pc}] 2: mult ${pos1}:${val1} * ${pos2}:${val2}, store ${result} at pos ${pos3}`);
      break;
    case 99:
      // end of program
      console.log(`[${pc}] 99: halt`);
      action = 'halt';
      break;
    default:
      // unknown opcode
      console.err(`[${pc}] Error: unknown opcode ${codes[pc]} at position ${pc}`);
      process.exit(1);
  }
  // default is to keep going
  return { action, intcodes: codes };
}

// read file as string
let intcodes_str = fs.readFileSync(INPUT_FILE, "utf-8");

// split on comma and convert to ints
let intcodes = intcodes_str.split(',').map(Number);

console.log("starting intcodes:");
console.log(intcodes);

// restore the "1202 program alarm" state
intcodes[1] = 12;
intcodes[2] = 2;

// run the program
let resulting_program = run_program(intcodes);

// log the resulting program at the end
console.log(resulting_program);
