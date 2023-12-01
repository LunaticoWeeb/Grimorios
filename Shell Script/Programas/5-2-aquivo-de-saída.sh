#!/usr/bin/env bash

echo Escreva algo:

read ENTRADA

echo $ENTRADA > teste.out

echo teste.out:

cat teste.out

echo Escreva novamente:

read ENTRADA

echo $ENTRADA > teste.out

echo teste.out:

cat teste.out

echo Escreva novamente:

read ENTRADA

echo $ENTRADA >> teste.out

echo teste.out:

cat teste.out

rm teste.out