#!/bin/sh
while read line; do
  echo $line | tr FBLR 0101 | sed -e "s/^/obase=10; ibase=2; /" | bc
done
