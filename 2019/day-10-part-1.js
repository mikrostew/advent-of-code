#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-10-input.txt';


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
  }

  // use Euclid's algorithm to scale this down to the lowest integer representation
  scaleDown() {
    // using abs because otherwise the negative will change the direction
    let gcd = findGCD(Math.abs(this.x), Math.abs(this.y));
    this.x /= gcd;
    this.y /= gcd;
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
          points.push(new Point(charIndex, lineIndex));
        }
      });
    });
    //console.log(`found ${points.length} points`);
    return points;
  }

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
}

// some test programs from the description:
let mapStr, map;

// The best location is 3,4 because it can detect 8 asteroids:
mapStr = `.#..#
.....
#####
....#
...##`;
map = new AsteroidMap(mapStr);
map.findBestLocation();
console.log("^^ expected (3,4) and 8 asteroids");

// Best is 5,8 with 33 other asteroids detected:
mapStr = `......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####`;
map = new AsteroidMap(mapStr);
map.findBestLocation();
console.log("^^ expected (5,8) and 33 asteroids");

// Best is 1,2 with 35 other asteroids detected:
mapStr = `#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.`;
map = new AsteroidMap(mapStr);
map.findBestLocation();
console.log("^^ expected (1,2) and 35 asteroids");

// Best is 6,3 with 41 other asteroids detected:
mapStr = `.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..`;
map = new AsteroidMap(mapStr);
map.findBestLocation();
console.log("^^ expected (6,3) and 41 asteroids");

// Best is 11,13 with 210 other asteroids detected:
mapStr = `.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##`;
map = new AsteroidMap(mapStr);
map.findBestLocation();
console.log("^^ expected (11,13) and 210 asteroids");


// input the program and run it
map = AsteroidMap.fromFile(INPUT_FILE);
map.findBestLocation();
