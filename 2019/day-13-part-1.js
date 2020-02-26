#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const IntcodeProgram = require('./intcode-program');


const INPUT_FILE = './day-13-input.txt';


// arcade thing
class ArcadeCabinet {
  constructor(programStr) {
    this.program = programStr;
  }

  static fromFile(file) {
    // read file as string
    let programStr = fs.readFileSync(file, "utf-8").trim();
    return new ArcadeCabinet(programStr);
  }
}


// input the program and run it
//map = AsteroidMap.fromFile(INPUT_FILE);
//map.findBestLocation();
