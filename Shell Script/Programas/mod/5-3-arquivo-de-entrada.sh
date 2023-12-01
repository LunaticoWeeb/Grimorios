#!/usr/bin/env bash

echo "Insira a entrada:"
read ENTRADA

echo $ENTRADA > teste.txt

wc -c < teste.txt

wc -w << EOF
Linha 1
Linha 2
Linha 3
Linha 4
EOF

wc -w <<< "WUIHDw  iuah hw uiw ahiu aiu wuih iuawi u"

rm teste.txt