#!/bin/bash

STRING=${1}
FILE=${2}

while read LINE ; do
    RESULT=$( echo "${LINE}" | grep -o "${STRING}" )
    if [ "${RESULT}" == "" ] ; then
        echo "Missing string!"
    else
        echo "Found string!"
    fi
done < ${FILE}
