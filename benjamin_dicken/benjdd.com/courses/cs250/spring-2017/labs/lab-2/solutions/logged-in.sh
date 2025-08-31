#!/bin/bash

USERNAMES_FILE=${1}
USERNAMES=$(cat ${USERNAMES_FILE})

for USERNAME in ${USERNAMES} ; do
    SESSION=$(who | grep -w -o ${USERNAME})
    if [ "${SESSION}" == ""  ]; then
        echo "${USERNAME} has no active shell sessions."
    else
        echo "${USERNAME} has one or more active shell sessions."
    fi
done

