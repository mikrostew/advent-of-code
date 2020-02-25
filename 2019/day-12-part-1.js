#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-12-input.txt';


// return all possible permutations (b/c we care about order) of 2 items in the input set of items
function getPermutations(items) {
  // can't do this with less than 2 items
  if (items.length < 2) { return []; }

  let permutations = [];
  let firstItem = items[0];
  let remainingItems = items.slice(1);
  remainingItems.forEach(item => permutations.push([firstItem, item]));
  // recursively get the rest of the possible permutations
  let otherPermutations = getPermutations(remainingItems);
  return permutations.concat(otherPermutations);
}

// a single moon
class Moon {
  constructor(x, y, z) {
    // set initial position - make sure these are numbers
    this.posX = Number(x);
    this.posY = Number(y);
    this.posZ = Number(z);
    // initial velocity is zero
    this.velX = 0;
    this.velY = 0;
    this.velZ = 0;
  }

  // update velocity of this moon for each dimension
  updateVelocity(x, y, z) {
    this.velX += x;
    this.velY += y;
    this.velZ += z;
  }

  // apply velocity to this moon's position
  applyVelocity() {
    this.posX += this.velX;
    this.posY += this.velY;
    this.posZ += this.velZ;
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
    //if (this.debug) { console.log(this.moons); }
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
      //if (this.debug) { console.log(`line: ${line}`); }
      let match = line.match(/<x=(-?\d+), y=(-?\d+), z=(-?\d+)>/);
      //if (this.debug) { console.log(match); }
      moons.push(new Moon(match[1], match[2], match[3]));
    });
    return moons;
  }

  simulateSteps(numSteps) {
    // the permutations will be the same every time, so take that out of the loop
    // (array of 0..N -- see https://stackoverflow.com/a/33352604)
    let moonIndices = Array.from(Array(this.moons.length).keys());
    let moonPermutations = getPermutations(moonIndices);
    //console.log("possible permutations:");
    //console.log(moonPermutations);
    if (this.debug) {
      console.log();
      console.log(`step 0:`);
      this.moons.forEach(m => console.log(m));
    }

    for (let i = 0; i < numSteps; i++) {
      // for each pair of moons, apply gravity to modify velocities
      moonPermutations.forEach(indices => {
        //console.log(`${indices[0]}, ${indices[1]}`);
        // the values that will be used to adjust velocity
        let m0Vel = [0, 0, 0];
        let m1Vel = [0, 0, 0];

        // adjust velocities
        let m0 = this.moons[indices[0]];
        let m1 = this.moons[indices[1]];
        // x
        if (m0.posX > m1.posX) {
          m0Vel[0] = -1;
          m1Vel[0] = 1;
        } else if (m0.posX < m1.posX) {
          m0Vel[0] = 1;
          m1Vel[0] = -1;
        } else {
          // equal, no change
        }
        // y
        if (m0.posY > m1.posY) {
          m0Vel[1] = -1;
          m1Vel[1] = 1;
        } else if (m0.posY < m1.posY) {
          m0Vel[1] = 1;
          m1Vel[1] = -1;
        } else {
          // equal, no change
        }
        // z
        if (m0.posZ > m1.posZ) {
          m0Vel[2] = -1;
          m1Vel[2] = 1;
        } else if (m0.posZ < m1.posZ) {
          m0Vel[2] = 1;
          m1Vel[2] = -1;
        } else {
          // equal, no change
        }
        m0.updateVelocity(...m0Vel);
        m1.updateVelocity(...m1Vel);
      });

      // after that, apply velocities to modify positions
      this.moons.forEach(m => m.applyVelocity());
      if (this.debug) {
        console.log();
        console.log(`step ${i+1}:`);
        this.moons.forEach(m => console.log(m));
      }
    }
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
