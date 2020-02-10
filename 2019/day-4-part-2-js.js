#!/usr/bin/env node

'use strict';

// (input is not a file this time, yay)
const lower_bound = 387638;
const upper_bound = 919123;

// because the digit comparison is more complicated now
function digit_matching_ok(d0, d1, d2, d3, d4, d5) {
  // check basic condition (maybe unnecessary but whatever)
  if (d0 == d1 || d1 == d2 || d2 == d3 || d3 == d4 || d4 == d5) {
    // check that there is some group of 2 digits not part of a larger group
    if (d0 == d1 && d1 != d2) { return true; }
    if (d0 != d1 && d1 == d2 && d2 != d3) { return true; }
    if (d1 != d2 && d2 == d3 && d3 != d4) { return true; }
    if (d2 != d3 && d3 == d4 && d4 != d5) { return true; }
    if (d3 != d4 && d4 == d5) { return true; }
    return false;
  } else {
    // doesn't even match the basic repeated digit criteria
    return false;
  }
}

// kinda brute force, but whatever,
// just generate all the possible numbers where the digits do not decrease,
// then check that each of those has at least one digit repeated

let possible_passwords = 0;

// this looks insane, but it's actually pretty fast because of the digit criteria
// (I could probably convert this to be recursive, but whatever)
for (let dig0 = 3; dig0 <= 8; dig0++) {
  for (let dig1 = dig0; dig1 <= 9; dig1++) {
    for (let dig2 = dig1; dig2 <= 9; dig2++) {
      for (let dig3 = dig2; dig3 <= 9; dig3++) {
        for (let dig4 = dig3; dig4 <= 9; dig4++) {
          for (let dig5 = dig4; dig5 <= 9; dig5++) {
            if (digit_matching_ok(dig0, dig1, dig2, dig3, dig4, dig5)) {
              let num = Number(`${dig0}${dig1}${dig2}${dig3}${dig4}${dig5}`);
              if (num >= lower_bound && num <= upper_bound) {
                // found one
                possible_passwords++;
                //console.log(`found ${dig0}${dig1}${dig2}${dig3}${dig4}${dig5} (${possible_passwords})`);
              }
            }
          }
        }
      }
    }
  }
}

console.log("");
console.log(`found ${possible_passwords} possible passwords`);
