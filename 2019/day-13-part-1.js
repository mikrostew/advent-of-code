#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const IntcodeProgram = require('./intcode-program');


const INPUT_FILE = './day-13-input.txt';

const TILES = {
  0: ' ', // empty
  1: '█', // wall
  2: '□', // block
  3: '—', // horizontal paddle
  4: 'O', // ball
}

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

  tileIdToChar(tileID) {
    return TILES[tileID];
  }

  putTileAtPosition(x, y, tileID) {
    // TODO
    process.stdout.write(`\x1b[${x};${y}H${this.tileIdToChar(tileID)}`);
  }
}


// input the program and run it
let arcade = ArcadeCabinet.fromFile(INPUT_FILE);
arcade.putTileAtPosition(1, 7, 1);
arcade.putTileAtPosition(1, 8, 2);
arcade.putTileAtPosition(1, 9, 3);
arcade.putTileAtPosition(1, 11, 4);
