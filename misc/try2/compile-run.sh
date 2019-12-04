#!/bin/bash

name=${1%.*}
nasm -f elf64 "$1"
ld "$name".o
./a.out
