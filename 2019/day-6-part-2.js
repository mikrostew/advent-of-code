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

  pathToCOM(object) {
    // find a path from the input object to COM, as an array
    let currentObject = object;
    let path = [currentObject];
    while (currentObject != 'COM') {
      currentObject = this.orbitMap[currentObject];
      // add to the beginning of the array
      path.unshift(currentObject);
    }
    console.log(path);
    return path;
  }

  numberOfTransfersToSanta() {
    // count the number of orbital transfers needed between YOU and SAN
    // which looks like finding the common ancestor
    // first find each path
    let youToCOM = this.pathToCOM('YOU');
    let sanToCOM = this.pathToCOM('SAN');
    // then step through and figure out where they diverge
    let shorterPathLength = youToCOM.length < sanToCOM.length ? youToCOM.length : sanToCOM.length;
    for (let i = 0; i < shorterPathLength; i++) {
      if (youToCOM[i] != sanToCOM[i]) {
        // paths have diverged, the previous node was the common ancestor
        // so calculate the length
        console.log(`remaining elements in YOU: ${youToCOM.length - i}`);
        console.log(`remaining elements in SAN: ${sanToCOM.length - i}`);
        // subtract one because fencepost
        return (youToCOM.length - i - 1) + (sanToCOM.length - i - 1);
      } else {
        //console.log(`'${youToCOM[i]}' is the same in both`);
      }
    }
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
//K)L
//K)YOU
//I)SAN`;
//const map = new OrbitalMap(inputMapStr);

console.log(`Total orbits: ${map.countAllOrbits()}`);

console.log(`Transfers to Santa: ${map.numberOfTransfersToSanta()}`);
