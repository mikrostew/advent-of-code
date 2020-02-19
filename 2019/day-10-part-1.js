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
    return `(${this.x}, ${this.y})`;
  }
}

// find a representation of a line between 2 input points
function calculateLine(p1, p2) {
  // check for divide by zero, and just use the x-value
  if (p1.x == p2.x) {
    return `x=${p1.x}`;
  }
  // TODO: do I need to represent this as a fraction? or is floating point precision enough for this?
  let slope = (p2.y - p1.y) / (p2.x - p1.x);
  // y = ax + b
  let y_intercept = p1.y - (slope * p1.x);
  // return this to use as a hash key
  return `${slope},${y_intercept}`;
}

// program that uses streams to read input and write output
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
    console.log(`found ${points.length} points`);
    return points;
  }

  findBestLocation() {
    let maxLines = 0;
    let bestPoint = undefined;
    // iterate thru all pairs of points, finding lines - most number of lines can see the most asteroids
    for (let i = 0; i < this.points.length; i++) {
      let lineMap = {};
      let currentPoint = this.points[i];
      for (let j = 0; j < this.points.length; j++) {
        // check yourself
        if (i == j) { continue; }
        // just add that the line exists, don't care about tracking which points are on it
        lineMap[calculateLine(currentPoint, this.points[j])] = 0;
      }
      let numLines = Object.keys(lineMap).length;
      console.log(`point ${currentPoint} --> found ${numLines} lines`);
      if (numLines > maxLines) {
        maxLines = numLines;
        bestPoint = currentPoint;
      }
    }
    console.log();
    console.log(`Best point is ${bestPoint}, with ${maxLines} asteroids detected`);
  }
}

// input the program and run it
//const map = AsteroidMap.fromFile(INPUT_FILE);

// some test programs from the description:
//
// The best location is 3,4 because it can detect 8 asteroids:
//const mapStr = `.#..#
//.....
//#####
//....#
//...##`;
//const map = new AsteroidMap(mapStr);
//
// TODO: this one is not right for me
// Best is 5,8 with 33 other asteroids detected:
//const mapStr = `......#.#.
//#..#.#....
//..#######.
//.#.#.###..
//.#..#.....
//..#....#.#
//#..#....#.
//.##.#..###
//##...#..#.
//.#....####`;
//const map = new AsteroidMap(mapStr);
//
// TODO: this gives the right point, but wrong number of asteroids
// Best is 1,2 with 35 other asteroids detected:
//const mapStr = `#.#...#.#.
//.###....#.
//.#....#...
//##.#.#.#.#
//....#.#.#.
//.##..###.#
//..#...##..
//..##....##
//......#...
//.####.###.`;
//const map = new AsteroidMap(mapStr);
//
// TODO: this one gives the wrong point
// Best is 6,3 with 41 other asteroids detected:
//const mapStr = `.#..#..###
//####.###.#
//....###.#.
//..###.##.#
//##.##.#.#.
//....###..#
//..#.#..#.#
//#..#.#.###
//.##...##.#
//.....#.#..`;
//const map = new AsteroidMap(mapStr);
//
// TODO: this one gives the wrong point
// Best is 11,13 with 210 other asteroids detected:
//const mapStr = `.#..##.###...#######
//##.############..##.
//.#.######.########.#
//.###.#######.####.#.
//#####.##.#.##.###.##
//..#####..#.#########
//####################
//#.####....###.#.#.##
//##.#################
//#####.##.###..####..
//..######..##.#######
//####.##.####...##..#
//.#####..#.######.###
//##...#.##########...
//#.##########.#######
//.####.#.###.###.#.##
//....##.##.###..#####
//.#.#.###########.###
//#.#.#.#####.####.###
//###.##.####.##.#..##`;
//const map = new AsteroidMap(mapStr);


map.findBestLocation();
