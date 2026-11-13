#!/bin/bash

for i in $(ls *.tape);
do
    vhs $i
done
