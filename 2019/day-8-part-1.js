#!/usr/bin/env node

'use strict';

const fs = require('fs');

const INPUT_FILE = './day-8-input.txt';
const IMAGE_WIDTH = 25;
const IMAGE_HEIGHT = 6;
//const IMAGE_WIDTH = 3;
//const IMAGE_HEIGHT = 2;

// TODO:
function countDigits(pixelData) {
}

// read the file as a string
let imageData = fs.readFileSync(INPUT_FILE, "utf-8").trim();
//let imageData = '123456789012';

let numberOfPixels = IMAGE_WIDTH * IMAGE_HEIGHT;
let currentLayer = 1;
let layerWithFewestZeros = 0;
let layerInfo = {};

while (imageData.length > 0) {
  // for each row, get the pixel data
  let [pixelData, remainingData] = [imageData.substring(0, numberOfPixels), imageData.slice(numberOfPixels)]
  //console.log(`layer ${currentLayer}:`);
  //console.log(pixelData);
  layerInfo[currentLayer] = { data: pixelData, digits: countDigits(pixelData) };
  // TODO: track least zeros

  // keep going
  imageData = remainingData;
  currentLayer++;
}
