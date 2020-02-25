#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-12-input.txt';


// simple point class
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  toString() {
    return `Point<${this.x},${this.y}>`;
  }
}

// a single moon
class Moon {
  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

}

// system of moons
class MoonSystem {
  constructor(positionStr, config) {
    // first set config
    if (config !== undefined) {
      this.debug = config.debug;
    }
    // then parse this info
    this.moons = this.parseMoonInfo(positionStr);
    if (this.debug) { console.log(this.moons); }
  }

  static fromFile(file) {
    // read file as string
    let positionStr = fs.readFileSync(file, "utf-8").trim();
    return new MoonSystem(positionStr);
  }

  parseMoonInfo(positionStr) {
    let moons = [];
    // input string looks like:
    // <x=-1, y=0, z=2>
    // <x=2, y=-10, z=-7>
    // <x=4, y=-8, z=8>
    // <x=3, y=5, z=-1>
    // split by lines, then use regex
    positionStr.split(/[\r\n]+/).forEach(line => {
      if (this.debug) { console.log(`line: ${line}`); }
      let match = line.match(/<x=(-?\d+), y=(-?\d+), z=(-?\d+)>/);
      //if (this.debug) { console.log(match); }
      moons.push(new Moon(match[1], match[2], match[3]));
    });
    return moons;
  }

  simulateSteps(numSteps) {
    // TODO
  }
}


// test programs from the description
let positionStr, moons;

positionStr = `<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>`;
moons = new MoonSystem(positionStr, {debug: true});
moons.simulateSteps(10);

// input the program and run it
//moons = MoonSystem.fromFile(INPUT_FILE);
//moons.simulateSteps(1000);
//console.log(`Total energy of the system: ${moons.systemEnergy()}`);
