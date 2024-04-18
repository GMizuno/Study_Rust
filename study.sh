#!/bin/bash

for i in {1..24}
do
touch src/cap$i/README.md
done

for i in {1..24}
do
 touch src/cap$i/cap$i.rs
done
