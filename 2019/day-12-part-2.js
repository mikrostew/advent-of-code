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

// find the least common multiple
function lcm(n1, n2) {
  // LCM is just product divided by gcd
  return (n1 * n2) / gcd(n1, n2);
}

// find the greatest common divisor
function gcd(n1, n2) {
  // base cases
  if (n1 === 0) { return n2; }
  if (n2 === 0) { return n1; }
  // otherwise continue the algorithm
  return gcd(n2, n1 % n2);
}

// a single moon
class Moon {
  constructor(x, y, z) {
    // set initial position - make sure these are numbers
    // for this part, use an object so I can more easily access in a loop
    this.pos = {
      x: Number(x),
      y: Number(y),
      z: Number(z),
    };
    // initial velocities are zero
    this.vel = {
      x: 0,
      y: 0,
      z: 0,
    };
  }

  // update velocity of this moon for each dimension
  updateVelocity(dimension, velocity) {
    this.vel[dimension] += velocity;
  }

  // apply velocity to this moon's position
  applyVelocity() {
    this.pos['x'] += this.vel['x'];
    this.pos['y'] += this.vel['y'];
    this.pos['z'] += this.vel['z'];
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

  // string representing the current state of the system, on a single axis
  stateString(dimension) {
    return `Pos<${this.moons.map(m => m.pos[dimension]).join('|')}> Vel<${this.moons.map(m => m.vel[dimension]).join('|')}>`;
  }

  // figure out how many steps this takes to repeat
  stepsToRepeat() {
    // independently figure out the repeat interval for each dimension (X, Y, and Z),
    // then find the LCM of that to get our answer

    // the permutations will be the same every time, so take that out of the loop
    // (array of 0..N -- see https://stackoverflow.com/a/33352604)
    let moonIndices = Array.from(Array(this.moons.length).keys());
    let moonPermutations = getPermutations(moonIndices);

    let stepsForEachDim = [];

    // do this for all 3 dimensions
    ['x', 'y', 'z'].forEach(currentDimension => {

      let i;
      let currentState = '';
      let initialState = this.stateString(currentDimension);
      console.log(`initial state: ${initialState}`);

      for (i = 0; currentState != initialState; i++) {
        // for each pair of moons, apply gravity to modify velocities
        moonPermutations.forEach(indices => {
          // the values that will be used to adjust velocity
          let m0Vel = 0;
          let m1Vel = 0;

          // adjust velocities
          let m0 = this.moons[indices[0]];
          let m1 = this.moons[indices[1]];
          // x
          if (m0.pos[currentDimension] > m1.pos[currentDimension]) {
            m0Vel = -1;
            m1Vel = 1;
          } else if (m0.pos[currentDimension] < m1.pos[currentDimension]) {
            m0Vel = 1;
            m1Vel = -1;
          } else {
            // equal, no change
          }

          m0.updateVelocity(currentDimension, m0Vel);
          m1.updateVelocity(currentDimension, m1Vel);
        });

        // after that, apply velocities to modify positions
        this.moons.forEach(m => m.applyVelocity());
        currentState = this.stateString(currentDimension);
      }

      console.log(`'${currentDimension}' took ${i} steps`);
      stepsForEachDim.push(i);
    });

    // figure out the LCM
    let totalSteps = lcm(stepsForEachDim[0], lcm(stepsForEachDim[1], stepsForEachDim[2]));
    console.log(`Total steps to repeat: ${totalSteps}`);
  }
}

// test programs from the description
let positionStr, moons;

// positionStr = `<x=-1, y=0, z=2>
// <x=2, y=-10, z=-7>
// <x=4, y=-8, z=8>
// <x=3, y=5, z=-1>`;
// moons = new MoonSystem(positionStr, {debug: true});
// moons.stepsToRepeat();

// positionStr = `<x=-8, y=-10, z=0>
// <x=5, y=5, z=10>
// <x=2, y=-7, z=3>
// <x=9, y=-8, z=-3>`;
// moons = new MoonSystem(positionStr, {debug: true});
// moons.stepsToRepeat();


// input the program and run it
moons = MoonSystem.fromFile(INPUT_FILE);
moons.stepsToRepeat();
