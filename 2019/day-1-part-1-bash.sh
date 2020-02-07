#!/usr/bin/env bash

INPUT_FILE="./day-1-input.txt"

total_fuel=0

while IFS= read -r line
do
  # bash automatically drops the remainder for division
  fuel=$(( line / 3 - 2))
  echo "mass: $line, fuel: $fuel"

  total_fuel=$(( total_fuel + fuel ))
done < "$INPUT_FILE"

echo "total: $total_fuel"
