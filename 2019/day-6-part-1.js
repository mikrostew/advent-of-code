#!/usr/bin/env node

'use strict';

const fs = require('fs');
//const process = require('process');

const INPUT_FILE = './day-6-input.txt';


class OrbitalMap {
  constructor(orbitMapStr) {
    this.orbitMap = {};
    // split on newline,
    // remove empty strings,
    // then split each of those on ')',
    // and add to the map
    orbitMapStr.split(/[\r\n]+/).filter(el => el != '').forEach(orbitPair => {
      let [orbited, orbiter] = orbitPair.split(')');
      //console.log(`${orbiter} orbits ${orbited}`);
      this.orbitMap[orbiter] = orbited;
    });
  }

  static fromFile(file) {
    // read file as string
    let orbitMapStr = fs.readFileSync(INPUT_FILE, "utf-8");
    return new OrbitalMap(orbitMapStr);
  }

  countAllOrbits() {
    let totalOrbits = 0;
    let currentObjectNumber = 1;
    let numberOfObjects = Object.keys(this.orbitMap).length;
    console.log(`There are ${numberOfObjects} objects in this graph`);
    // for each thing, walk back to COM, counting each step as an orbit
    // NOTE: this is O(x^2), but it is simple, and fast enough for this
    Object.keys(this.orbitMap).forEach(object => {
      let currentObject = object;
      //console.log(`Object '${currentObject}' (${currentObjectNumber}/${numberOfObjects})`);
      while (currentObject != 'COM') {
        currentObject = this.orbitMap[currentObject];
        totalOrbits++;
      }
      currentObjectNumber++;
    });
    return totalOrbits;
  }
}

// input the program and run it
const map = OrbitalMap.fromFile(INPUT_FILE);

// sample input from the page
//let inputMapStr = `COM)B
//B)C
//C)D
//D)E
//E)F
//B)G
//G)H
//D)I
//E)J
//J)K
//K)L`;
//const map = new OrbitalMap(inputMapStr);

console.log(`Total orbits: ${map.countAllOrbits()}`);
