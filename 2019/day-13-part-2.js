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
    this.gameInput = new IntcodeInOutStream();
    this.gameOutput = new IntcodeInOutStream();
    // this is a bit of a hack, because I know the first char is '1'
    let newProgramStr = programStr.replace('1', '2'); // put in the quarters
    this.program = new IntcodeProgram(newProgramStr, this.gameInput, this.gameOutput);
    //this.program = new IntcodeProgram(newProgramStr, process.stdin, this.gameOutput);
    //this.joystickPosition = 0;
    // track ball and paddle positions (columns)
    this.ballX = 0;
    this.paddleX = 0;
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
    // track ball and paddle positions
    if (tileID === 3) {
      this.paddleX = x;
    } else if (tileID === 4) {
      this.ballX = x;
    }
    // the order here is line then column, which is why Y is first
    process.stdout.write(`\x1b[${y};${x}H${this.tileIdToChar(tileID)}`);
  }

  putTextAtPosition(x, y, text) {
    // the order here is line then column, which is why Y is first
    process.stdout.write(`\x1b[${y};${x}H${text}`);
  }

  // this was a nice idea
  // setupJoystickInput() {
  //   // don't wait for enter to be pressed to get characters
  //   process.stdin.setRawMode(true);
  //
  //   // resume stdin in the parent process
  //   // (so this won't quit unless an error or process.exit() happens)
  //   process.stdin.resume();
  //
  //   // get hex values (makes reading the arrow key values easier)
  //   process.stdin.setEncoding('hex');
  //
  //   // on any data into stdin
  //   process.stdin.on('data', key => {
  //     // Ctrl-C or Ctrl-D (end of text) exits this program
  //     if (key === '03' || key === '04') {
  //       process.exit();
  //     }
  //
  //     if (key === '1b5b44') { // left
  //       this.joystickPosition = -1;
  //     } else if (key === '1b5b43') { // right
  //       this.joystickPosition = 1;
  //     }
  //   });
  // }

  setupGameLoop() {
    // new frame of the game every 1/2 second
    setInterval(() => {
      // all I have to do is read what the joystick input is, then write that to the game
      //let jp = this.joystickPosition;
      //this.gameInput.write(`${jp}\n`);
      // and reset the position
      //this.joystickPosition = 0;

      // no ^^
      // figure out where to move the paddle based on the positions of the paddle and ball
      if (this.ballX > this.paddleX) {
        // move right
        this.gameInput.write(`1\n`);
      } else if (this.ballX < this.paddleX) {
        // move left
        this.gameInput.write(`-1\n`);
      } else {
        // same column, probably have to anticipate the ball's movements but whatever
        this.gameInput.write(`0\n`);
      }
    }, 500);
  }

  async runGame() {
    // handle output from the game
    const readGameOutput = readline.createInterface({
      input: this.gameOutput,
      output: undefined, // so that it doesn't echo the outputs it receives, WTF
    });

    let [xPos, yPos, tileIDOrScore] = [undefined, undefined, undefined];
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
        tileIDOrScore = Number(input);
        // now we have all the inputs, so draw on the screen
        // unless this is a segment display command (the current score)
        if (xPos === -1 && yPos === 0) {
          this.putTextAtPosition(0, maxY+1, `score: ${tileIDOrScore}`); // this is actually the current score
        } else {
          //console.log(`put tile ${tileIDOrScore} at position ${xPos},${yPos}`);
          //if (tileIDOrScore == 2) { numBlocks++; }
          this.putTileAtPosition(xPos, yPos, tileIDOrScore);
        }
        // clear these out to start over
        [xPos, yPos, tileIDOrScore] = [undefined, undefined, undefined];
      }
    });

    // setup input from the arrow keys
    //this.setupJoystickInput();

    // clear the screen first
    this.clearScreen();

    // then start the main game loop
    this.setupGameLoop();

    // don't have to send any input, at least not this time...
    await this.program.run();

    // move the cursor after running this (add a blank tile below the game and the score)
    this.putTileAtPosition(0, maxY+2, 0);
    //setTimeout(() => console.log(`number of blocks: ${numBlocks}`), 500);

    // close this or it will hang?
    //readGameOutput.close();
    // or I could do this
    process.exit();
  }
}


// input the program and run it
let arcade = ArcadeCabinet.fromFile(INPUT_FILE);
//let arcade = new ArcadeCabinet('104,1,104,2,104,3,104,6,104,5,104,4,99');
arcade.runGame();
