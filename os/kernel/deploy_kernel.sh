#!/bin/bash
BASEDIR=$(dirname "$0")

KERNEL=./build/kernel.bin
TTYPATH=/dev/ttyUSB0
BAUD=115200

cd "$BASEDIR"

make clean && make all

if [ $? -eq 0 ]; then
    ttywrite -i "$KERNEL" "$TTYPATH"
    if [ $? -eq 0 ]; then
        screen $TTYPATH $BAUD
    else
        echo "Failed to deploy kernel via ttywrite."
        exit $?
    fi
else
    echo "Kernel failed to build."
    exit $?
fi