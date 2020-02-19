#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-10-input.txt';


class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
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
    return points;
  }

  findBestLocation() {
    // TODO
  }
}

// input the program and run it
//const map = AsteroidMap.fromFile(INPUT_FILE);

// some test programs from the description:
//
// The best location is 3,4 because it can detect 8 asteroids:
const mapStr = `.#..#
.....
#####
....#
...##`;
const map = new AsteroidMap(mapStr);
//
// Best is 5,8 with 33 other asteroids detected:
// const mapStr = `......#.#.
// #..#.#....
// ..#######.
// .#.#.###..
// .#..#.....
// ..#....#.#
// #..#....#.
// .##.#..###
// ##...#..#.
// .#....####`;
//
// Best is 1,2 with 35 other asteroids detected:
// const mapStr = `#.#...#.#.
// .###....#.
// .#....#...
// ##.#.#.#.#
// ....#.#.#.
// .##..###.#
// ..#...##..
// ..##....##
// ......#...
// .####.###.`;
//
// Best is 6,3 with 41 other asteroids detected:
// const mapStr = `.#..#..###
// ####.###.#
// ....###.#.
// ..###.##.#
// ##.##.#.#.
// ....###..#
// ..#.#..#.#
// #..#.#.###
// .##...##.#
// .....#.#..`;
//
// Best is 11,13 with 210 other asteroids detected:
// const mapStr = `.#..##.###...#######
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


map.findBestLocation();
