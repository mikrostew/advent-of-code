#!/usr/bin/env node

'use strict';

const fs = require('fs');

const INPUT_FILE = './day-8-input.txt';
const IMAGE_WIDTH = 25;
const IMAGE_HEIGHT = 6;
// const IMAGE_WIDTH = 2;
// const IMAGE_HEIGHT = 2;

// read the file as a string
let imageData = fs.readFileSync(INPUT_FILE, "utf-8").trim();
// let imageData = '0222112222120000';

let numberOfPixels = IMAGE_WIDTH * IMAGE_HEIGHT;
let currentLayer = 1;
// initialize an array filled with 2's
let finalPixels = Array.from({length: numberOfPixels}, () => '2');

while (imageData.length > 0) {
  // for each row, get the pixel data
  let [pixelData, remainingData] = [imageData.substring(0, numberOfPixels), imageData.slice(numberOfPixels)]
  //console.log(`layer ${currentLayer}: ${pixelData}`);

  // iterate through each pixel, and set any non-transparent ones that have not already been set
  pixelData.split('').forEach((p, i) => {
    if (p !== '2') {
      if (finalPixels[i] == '2') {
        // blank for black pixels, full block for white ones (easier to see)
        finalPixels[i] = (p == 0 ? ' ' : '█');
      }
    }
  });

  // keep going
  imageData = remainingData;
  currentLayer++;
}

//console.log(`decoded result:`);
//console.log(finalPixels);

// print it nicely
console.log();
for (let i = 0; i < finalPixels.length; i += IMAGE_WIDTH) {
  console.log(finalPixels.slice(i, i + IMAGE_WIDTH).join(''));
}
