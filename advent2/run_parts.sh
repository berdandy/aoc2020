#!/bin/sh

echo "Part1\n-----"
awk -F'[: -][ ]*' 'BEGIN { s = 0 } { chk = $3 ; cnt=gsub($3,"", $4); if (cnt >= $1 && cnt <= $2) s += 1 } END { print s }' data.txt

echo "\nPart2\n-----"
awk -F'[: -][ ]*' 'BEGIN { s = 0 } { chk = $3 ; a=substr($4,$1,1) ~ chk; b=substr($4,$2,1) ~ chk ; if ((a || b) && !(a && b)) s += 1 } END { print s }' data.txt
