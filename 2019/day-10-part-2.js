#!/usr/bin/env node

'use strict';

const assert = require('assert');
const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-10-input.txt';


// simple point class
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  equals(anotherPoint) {
    return (this.x == anotherPoint.x && this.y == anotherPoint.y);
  }

  toString() {
    return `Point<${this.x},${this.y}>`;
  }
}

// find the greatest common divisor of two integers
function findGCD(n1, n2) {
  // base cases
  if (n1 == 0) { return n2; }
  if (n2 == 0) { return n1; }
  return findGCD(n2, n1 % n2);
}

// vector between 2 points
class Vector {
  constructor(p1, p2) {
    this.x = p2.x - p1.x;
    this.y = p2.y - p1.y;
    // save start and end points
    this.startPoint = p1;
    this.endPoint = p2;
    // calculate magnitude, and angle in radians
    this.magnitude = Math.sqrt(this.x * this.x + this.y * this.y);
    this.angle = Math.atan2(this.x, this.y);
    // (adjust to be positive, because atan2 uses negative radians for these and that doesn't work well here)
    if (this.x < 0) {
      this.angle += 2 * Math.PI;
    }
  }

  // use Euclid's algorithm to scale this down to the lowest integer representation
  scaleDown() {
    // using abs because otherwise the negative will change the direction
    let gcd = findGCD(Math.abs(this.x), Math.abs(this.y));
    this.x /= gcd;
    this.y /= gcd;
  }

  // sort by angle, then use magnitude if angles are equal
  compare(v2) {
    if (this.angle < v2.angle) { return -1; }
    if (this.angle > v2.angle) { return 1; }
    // angles are equal, compare the magnitudes
    if (this.magnitude < v2.magnitude) { return -1; }
    if (this.magnitude > v2.magnitude) { return 1; }
    // they are equal
    return 0;
  }

  toString() {
    return `Vector<${this.x},${this.y}`;
  }
}

// map of asteroids that need to be tracked
class AsteroidMap {
  constructor(mapStr) {
    this.points = this.parseToPoints(mapStr);
  }

  static fromFile(file) {
    // read file as string
    let mapStr = fs.readFileSync(file, "utf-8").trim();
    return new AsteroidMap(mapStr);
  }

  // parse the ASCII characters to x,y coordinates
  parseToPoints(mapStr) {
    let points = [];
    // split by lines
    mapStr.split(/[\r\n]+/).forEach((line, lineIndex) => {
      // then split by chars
      line.split('').forEach((c, charIndex) => {
        // check for asteroid and add its position
        if (c == '#') {
          // y-coords are negated so this aligns to the normal cartesian plane, where radians work nicely
          points.push(new Point(charIndex, lineIndex * -1));
        }
      });
    });
    //console.log(`found ${points.length} points`);
    return points;
  }

  // NOTE: not using this function this time
  findBestLocation() {
    let maxAsteroids = 0;
    let bestPoint = undefined;
    // iterate thru all pairs of points, finding vectors - most number of vectors can see the most asteroids
    for (let i = 0; i < this.points.length; i++) {
      let vectorMap = {};
      let currentPoint = this.points[i];
      for (let j = 0; j < this.points.length; j++) {
        // check yourself
        if (i == j) { continue; }
        let vector = new Vector(currentPoint, this.points[j]);
        // reduce vectors so that asteroids that cannot be seen will map to the same vector as the one in front
        vector.scaleDown();
        // just mark that the vector exists, don't care exactly what points are on it
        vectorMap[vector.toString()] = 0;
      }
      // the number of asteroids that can be seen from this asteroid
      let numAsteroids = Object.keys(vectorMap).length;
      //console.log(`asteroid ${currentPoint} --> can see ${numAsteroids} asteroids`);
      if (numAsteroids > maxAsteroids) {
        maxAsteroids = numAsteroids;
        bestPoint = currentPoint;
      }
    }
    console.log();
    console.log(`Best asteroid is ${bestPoint}, which can detect ${maxAsteroids} asteroids`);
  }

  // get the order that the asteroids would be vaporized
  // starting at 12:00, and proceeding clockwise
  getVaporizationOrder(stationPosition) {
    // figure out all the vectors to the other asteroids
    let allVectors = [];
    for (let i = 0; i < this.points.length; i++) {
      // skipping the station position point
      if (!this.points[i].equals(stationPosition)) {
        let vector = new Vector(stationPosition, this.points[i]);
        allVectors.push(vector);
      }
    }
    // sort those by the angle, starting at 0 radians (12:00)
    let sortedVectors = allVectors.sort((a, b) => a.compare(b));
    // TODO: this is not right yet, but print things out as a check
    sortedVectors.forEach((v, i) => {
      console.log(`${i}: ${v.endPoint}`);
    });
    return [];
  }
}

// some test programs from the description:
let mapStr, stationPos, map;

// For example, consider the following map, where the asteroid with the new monitoring station (and laser) is marked X:
mapStr = `.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##`;
stationPos = new Point(8, -3);
map = new AsteroidMap(mapStr);
map.getVaporizationOrder(stationPos);

// TODO:
// The first nine asteroids to get vaporized, in order, would be:
//
// .#....###24...#..
// ##...##.13#67..9#
// ##...#...5.8####.
// ..#.....X...###..
// ..#.#.....#....##
//
// which is: (8,1) (9,0) (9,1) (10,0) (9,2) (11,1) (12,1) (11,2) (15,1)
//
// Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7) won't have a chance to be vaporized until the next full rotation. The laser continues rotating; the next nine to be vaporized are:
//
// .#....###.....#..
// ##...##...#.....#
// ##...#......1234.
// ..#.....X...5##..
// ..#.9.....8....76
//
// which is: (12,2) (13,2) (14,2) (15,2) (12,3) (16,4) (15,4) (10,4) (4,4)
//
// The next nine to be vaporized are then:
//
// .8....###.....#..
// 56...9#...#.....#
// 34...7...........
// ..2.....X....##..
// ..1..............
//
// which is: (2,4) (2,3) (0,2) (1,2) (0,1) (1,1) (5,2) (1,0) (5,1)
//
// Finally, the laser completes its first full rotation (1 through 3), a second rotation (4 through 8), and vaporizes the last asteroid (9) partway through its third rotation:
//
// ......234.....6..
// ......1...5.....7
// .................
// ........X....89..
// .................
//
// which is: (6,1) (6,0) (7,0) (8,0) (10,1) (14,0) (16,1) (13,3) (14,3)

// In the large example above (the one with the best monitoring station location at 11,13):
// mapStr = `.#..##.###...#######
// ##.############..##.
// .#.######.########.#
// .###.#######.####.#.
// #####.##.#.##.###.##
// ..#####..#.#########
// ####################
// #.####....###.#.#.##
// ##.#################
// #####.##.###..####..
// ..######..##.#######
// ####.##.####...##..#
// .#####..#.######.###
// ##...#.##########...
// #.##########.#######
// .####.#.###.###.#.##
// ....##.##.###..#####
// .#.#.###########.###
// #.#.#.#####.####.###
// ###.##.####.##.#..##`;
// stationPos = new Point(11, -13);
// map = new AsteroidMap(mapStr);
// map.getVaporizationOrder(stationPos);

// TODO:
// The 1st asteroid to be vaporized is at 11,12.
// The 2nd asteroid to be vaporized is at 12,1.
// The 3rd asteroid to be vaporized is at 12,2.
// The 10th asteroid to be vaporized is at 12,8.
// The 20th asteroid to be vaporized is at 16,0.
// The 50th asteroid to be vaporized is at 16,9.
// The 100th asteroid to be vaporized is at 10,16.
// The 199th asteroid to be vaporized is at 9,6.
// The 200th asteroid to be vaporized is at 8,2.
// The 201st asteroid to be vaporized is at 10,9.
// The 299th and final asteroid to be vaporized is at 11,1.

// TODO:
// input the program and run it
// map = AsteroidMap.fromFile(INPUT_FILE);
// stationPos = new Point(22, -25); // answer from last time
// map.getVaporizationOrder(stationPos);
