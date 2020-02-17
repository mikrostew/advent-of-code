#!/usr/bin/env node

'use strict';

// this should probably use classes and such to be nicer, but whatever

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-2-input.txt';
//const TARGET_VALUE = 2890696; // for noun=12, verb=2
const TARGET_VALUE = 19690720;

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

  //console.log(`[${pc}] ${codes[pc]} ${codes[pc+1]} ${codes[pc+2]} ${codes[pc+3]}`);

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

      //console.log(`[${pc}] 1: add ${pos1}:${val1} + ${pos2}:${val2}, store ${result} at pos ${pos3}`);
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

      //console.log(`[${pc}] 2: mult ${pos1}:${val1} * ${pos2}:${val2}, store ${result} at pos ${pos3}`);
      break;
    case 99:
      // end of program
      //console.log(`[${pc}] 99: halt`);
      action = 'halt';
      break;
    default:
      // unknown opcode
      console.error(`[${pc}] Error: unknown opcode ${codes[pc]} at position ${pc}`);
      process.exit(1);
  }
  // default is to keep going
  return { action, intcodes: codes };
}

// read file as string
let intcodes_str = fs.readFileSync(INPUT_FILE, "utf-8");

// split on comma and convert to ints
const intcodes = intcodes_str.split(',').map(Number);

//console.log("starting intcodes:");
//console.log(intcodes);

// iterate these inputs to find the target output value
let noun, verb;
for (noun = 0; noun < 100; noun++) {
  for (verb = 0; verb < 100; verb++) {

    console.log(`trying noun=${noun}, verb=${verb}`);
    const fresh_intcodes = [...intcodes];

    // initialize the state
    fresh_intcodes[1] = noun;
    fresh_intcodes[2] = verb;

    // run the program
    let resulting_program = run_program(fresh_intcodes);

    if (resulting_program[0] == TARGET_VALUE) {
      console.log(`Found target value ${TARGET_VALUE} with noun=${noun}, verb=${verb}`);
      process.exit(0);
    }
  }
}

// Found target value 19690720 with noun=82, verb=26
