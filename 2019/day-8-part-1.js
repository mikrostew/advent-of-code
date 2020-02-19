#!/usr/bin/env node

'use strict';

const fs = require('fs');

const INPUT_FILE = './day-8-input.txt';
const IMAGE_WIDTH = 25;
const IMAGE_HEIGHT = 6;
//const IMAGE_WIDTH = 3;
//const IMAGE_HEIGHT = 2;

// return the count of each digit in this layer
function countDigits(pixelData) {
  let digitInfo = {};
  pixelData.split('').forEach(d => {
    if (digitInfo[d] === undefined) {
      // initialize the count
      digitInfo[d] = 0;
    }
    digitInfo[d]++;
  });
  return digitInfo;
}

// read the file as a string
let imageData = fs.readFileSync(INPUT_FILE, "utf-8").trim();
//let imageData = '123456789012';

let numberOfPixels = IMAGE_WIDTH * IMAGE_HEIGHT;
console.log(`pixels per layer: ${numberOfPixels}`);
let currentLayer = 1;
let layerWithFewestZeros = 0;
let layerInfo = {};
let leastZeroLayer = 0; // layers start at 1

while (imageData.length > 0) {
  // for each row, get the pixel data
  let [pixelData, remainingData] = [imageData.substring(0, numberOfPixels), imageData.slice(numberOfPixels)]
  console.log(`layer ${currentLayer}:`);
  console.log(pixelData);
  let digitInfo = countDigits(pixelData);
  layerInfo[currentLayer] = { data: pixelData, digits: digitInfo };
  console.log(digitInfo);
  // track least zeros
  if (leastZeroLayer == 0) {
    leastZeroLayer = currentLayer;
  } else if (layerInfo[currentLayer].digits[0] < layerInfo[leastZeroLayer].digits[0]) {
    leastZeroLayer = currentLayer;
  }

  // keep going
  imageData = remainingData;
  currentLayer++;
}

let layerDigits = layerInfo[leastZeroLayer].digits;

console.log(`layer with the least zeros: ${leastZeroLayer} (${layerDigits[0]} zeros)`);
console.log(`ones: ${layerDigits[1]}, twos: ${layerDigits[2]}`);
console.log(`${layerDigits[1]} * ${layerDigits[2]} = ${layerDigits[1] * layerDigits[2]}`);
