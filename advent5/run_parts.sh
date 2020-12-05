#!/bin/sh

echo "Part1\n-----"
cat data.txt | ./advent5.sh | sort -n | tail -1

echo "\nPart2\n-----"
cat data.txt | ./advent5.sh | sort -n | awk '{ print (NR+90) " - " $1 }' | grep -v '\([0-9][0-9]*\) - \1' | head -1 | cut -c 1-3
