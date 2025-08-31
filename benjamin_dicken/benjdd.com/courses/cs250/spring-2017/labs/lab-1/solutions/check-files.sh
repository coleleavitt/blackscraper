#!/bin/bash

# The first command-line option is a string to search for
STRING=${1}

for FILE in $( find . -maxdepth 1 -type f ) ; do
  IN=$( grep -i ${STRING} ${FILE} )
  if [ -z "${IN}" ]; then
      echo "${FILE} is missing the string: ${STRING}"
  else
      echo "${FILE} has the string: ${STRING}"
  fi
done

