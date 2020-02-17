#!/usr/bin/env node

'use strict';

const fs = require('fs');
const process = require('process');

const INPUT_FILE = './day-3-input.txt';


// a point on the grid with integer coordinates
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  get distance() {
    return Math.abs(this.x) + Math.abs(this.y);
  }

  toString() {
    return `${this.x},${this.y}`;
  }
}

// a segment on the path, with a direction and length
class Segment {
  constructor(segment_str) {
    // direction is the first character
    this.direction = segment_str.substring(0, 1);
    // the rest are the distance
    this.length = Number(segment_str.substring(1));
  }

  points(start_point) {
    // a range to use for creating the points, starting from 1
    const point_range = [...Array(this.length).keys()].map(i => i + 1);

    switch (this.direction) {
      case 'U':
        // increment y
        return point_range.map(i => new Point(start_point.x, start_point.y + i));
        break;
      case 'L':
        // decrement x
        return point_range.map(i => new Point(start_point.x - i, start_point.y));
        break;
      case 'D':
        // decrement y
        return point_range.map(i => new Point(start_point.x, start_point.y - i));
        break;
      case 'R':
        // increment x
        return point_range.map(i => new Point(start_point.x + i, start_point.y));
        break;
      default:
        console.error(`Unknown direction ${this.direction}`);
        process.exit(1);
    }
  }
}


// read file as string
let path_strings = fs.readFileSync(INPUT_FILE, "utf-8");

// split by lines into the two paths
// (see https://stackoverflow.com/a/21895354)
let paths = path_strings.split(/[\r\n]+/)

// split into the path for each line
let path1 = paths[0].split(',');
let path2 = paths[1].split(',');

//console.log(path1);
//console.log(path2);

let path1_points = {};

// start at origin
let current_point = new Point(0, 0);

// get the points on each segment, and add those to the set
path1.forEach(segment_str => {
  const segment = new Segment(segment_str);
  let points = segment.points(current_point);
  points.forEach(p => { path1_points[p] = p; });

  // finally, update the current point to be the end of the last segment
  current_point = points[points.length - 1];
});

//console.log(path1_points);

// they will both cross at the origin of course, but we won't count this
let min_distance = 0;

// and start back at the origin again
current_point = new Point(0, 0);

// now, check each point on path2 to see if it intersects, and if so what is the distance
path2.forEach(segment_str => {
  const segment = new Segment(segment_str);
  let points = segment.points(current_point);

  points.forEach(p => {
    if (path1_points[p] !== undefined) {
      console.log(`found intersection at ${p.x}, ${p.y}`);

      // found an intersection - is this the first one, or the shortest one?
      if (min_distance == 0 || p.distance < min_distance) {
        min_distance = p.distance;
      }
    }
  });

  // finally, update the current point to be the end of the last segment
  current_point = points[points.length - 1];
});

console.log('');
console.log(`intersection minimum distance: ${min_distance}`);

