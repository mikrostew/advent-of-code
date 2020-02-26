#!/usr/bin/env node

'use strict';

//////////////////////////////
// get input one char at a time from stdin
// (adapted from https://stackoverflow.com/a/12506613)
//////////////////////////////

const process = require('process');

const INPUT_STREAM = process.stdin;

// without this, we would only get streams once enter is pressed
INPUT_STREAM.setRawMode(true);

// resume stdin in the parent process (node app won't quit all by itself
// unless an error or process.exit() happens)
INPUT_STREAM.resume();

// I want hex strings, since I need to capture arrow keys
// (which don't work with utf-8)
INPUT_STREAM.setEncoding( 'hex' );

// keypress will trigger this event
INPUT_STREAM.on( 'data', function( key ){
  // Exit on Ctrl-C
  if (key === '03') {
    process.stdout.write(`keypress: '${key}' (Ctrl-C)\n`);
    process.exit();
  }
  // Exit on Ctrl-D (end of text)
  if (key === '04') {
    process.stdout.write(`keypress: '${key}' (Ctrl-D)\n`);
    process.exit();
  }

  // what the arrow keys are
  if (key === '1b5b44') { // left
    process.stdout.write(`keypress: '${key}' (left)\n`);
  }
  if (key === '1b5b41') { // up
    process.stdout.write(`keypress: '${key}' (up)\n`);
  }
  if (key === '1b5b43') { // right
    process.stdout.write(`keypress: '${key}' (right)\n`);
  }
  if (key === '1b5b42') { // down
    process.stdout.write(`keypress: '${key}' (down)\n`);
  }

  // write the key out, to figure out what the arrow keys are
  process.stdout.write(`keypress: '${key}'\n`);
});
