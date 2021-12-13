#!/bin/bash

for i in `seq 1 5`;
do
    source ./interaction/devnet.snippets.sh; createInstance
    sleep 5
done