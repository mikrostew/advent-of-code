#!/usr/bin/env bash

INPUT_FILE="./day-1-input.txt"

total_fuel=0

fuel_for_weight() {
  input_weight="$1"

  # formula for how much fuel is needed
  # (bash automatically drops the remainder for division)
  echo $(( input_weight / 3 - 2 ))
}

while IFS= read -r line
do
  mass="$line"

  echo -n "mass: $mass"

  fuel=$(fuel_for_weight "$mass")
  echo -n ", fuel: $fuel"

  # also have to calculate the fuel needed for that fuel, and for that fuel, etc...
  while [ "$fuel" -gt 0 ]
  do
    total_fuel=$(( total_fuel + fuel ))

    fuel=$(fuel_for_weight "$fuel")
    echo -n ", $fuel"
  done

  # don't forget the newline
  echo ""

done < "$INPUT_FILE"

echo "total: $total_fuel"
