#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');
const readline = require('readline');
const IntcodeProgram = require('./intcode-program');
const IntcodeInOutStream = require('./intcode-in-out-stream');


const INPUT_FILE = './day-13-input.txt';

const TILES = {
  0: ' ', // empty
  1: '█', // wall
  2: '□', // block
  3: '—', // horizontal paddle
  4: 'o', // ball
}

// arcade thing
class ArcadeCabinet {
  constructor(programStr) {
    // setup output stream
    this.gameOutput = new IntcodeInOutStream();
    this.program = new IntcodeProgram(programStr, process.stdin, this.gameOutput);
  }

  static fromFile(file) {
    // read file as string
    let programStr = fs.readFileSync(file, "utf-8").trim();
    return new ArcadeCabinet(programStr);
  }

  tileIdToChar(tileID) {
    return TILES[tileID];
  }

  clearScreen() {
    process.stdout.write('\x1b[2J');
  }

  putTileAtPosition(x, y, tileID) {
    // the order here is line then column, which is why Y is first
    process.stdout.write(`\x1b[${y};${x}H${this.tileIdToChar(tileID)}`);
  }

  async runGame() {
    // handle output from the game
    const readGameOutput = readline.createInterface({
      input: this.gameOutput,
      output: undefined, // so that it doesn't echo the outputs it receives, WTF
    });

    let [xPos, yPos, tileID] = [undefined, undefined, undefined];
    // track height of the game area
    let maxY = 0;
    // track number of blocks, see if it matches
    //let numBlocks = 0;

    readGameOutput.on('line', input => {
      // figure out which input this is
      if (xPos == undefined) {
        // just capture the x position
        xPos = Number(input);
      } else if (yPos == undefined) {
        // capture y position, the next thing
        yPos = Number(input);
        if (yPos > maxY) { maxY = yPos; }
      } else {
        // assume I did this right
        tileID = Number(input);
        // now we have all the inputs, so draw on the screen
        //console.log(`put tile ${tileID} at position ${xPos},${yPos}`);
        //if (tileID == 2) { numBlocks++; }
        this.putTileAtPosition(xPos, yPos, tileID);
        // clear these out to start over
        [xPos, yPos, tileID] = [undefined, undefined, undefined];
      }
    });

    // clear the screen first
    this.clearScreen();

    // don't have to send any input, at least not this time...
    await this.program.run();

    // TODO: wait for output stream to be cleared out

    // move the cursor after running this (add a blank tile below the game)
    this.putTileAtPosition(0, maxY+1, 0);
    //setTimeout(() => console.log(`number of blocks: ${numBlocks}`), 500);

    // close this or it will hang?
    //readGameOutput.close();
  }
}


// input the program and run it
let arcade = ArcadeCabinet.fromFile(INPUT_FILE);
//let arcade = new ArcadeCabinet('104,1,104,2,104,3,104,6,104,5,104,4,99');
arcade.runGame();


// this is the output:
let gameOutput = `                                        █
 □   □□□ □□□□  □□□  □□ □□ □ □       □ □ █
  □        □   □□□ □  □  □□ □□  □  □□   █
     □□□□□ □ □ □   □    □□  □  □    □   █
 □□   □  □□□□ □□ □□□   □  □□□  □□□  □□  █
 □□ □□□ □□ □ □ □□□□  □ □□ □□    □ □  □□ █
  □□ □□  □□  □□  □□□  □ □  □ □□□□□□ □   █
   □□  □ □ □□□□□□□ □□□    □ □ □ □ □ □□  █
  □□□□□□□ □□□ □ □ □□  □    □□  □ □   □  █
 □  □□□□□ □   □ □ □□□□□□□□ □□□  □□□□□   █
    □□□□□ □   □     □□    □□   □  □□□□  █
    □  □□□□  □□□□□□    □□ □  □□□□ □ □□□ █
 □ □ □  □  □   □□   □□ □   □□□□□        █
  □□□□□□ □  □   □ □□ □  □□□□  □□□□ □□□□ █
  □    □□  □□ □ □  □□□□   □  □□ □□ □ □  █
  □ □□□   □□□  □ □□      □□□ □□ □    □□ █
 □□□□□ □  □ □□□□    □ □□ □□□   □  □□□ □ █
                                        █
                  o                     █
                                        █
                                        █
                    —                   █
                                        █`;

let numBlocks = 0;
gameOutput.split('').forEach(c => {
  if (c == '□') {
    numBlocks++;
  }
});
console.log(`Blocks on screen: ${numBlocks}`);
