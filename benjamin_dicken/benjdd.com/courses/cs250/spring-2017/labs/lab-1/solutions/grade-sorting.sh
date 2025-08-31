#!/bin/bash

cat grades.txt | sort -k 2 > grades-sorted-fn.txt

cat grades.txt | sort -k 3 -r > grades-sorted-ln.txt

cat grades.txt | sort -k 1 -n | head -n 3 | sort -k 4 > low-grades-sorted-email.txt

cat grades.txt | sort -k 1 -n | tail -n 4 | sort -k 3 > high-grades-sorted-ln.txt

cat grades.txt | grep gmail | cut -d " " -f 4 | sort > sorted-gmail.txt

