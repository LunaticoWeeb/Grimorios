#!/usr/bin/env bash

echo Qual tipo de arquivo você quer listar?

read TIPO

echo Aqui estão os arquivos do tipo $TIPO:

ls . | grep $TIPO #aqui poderia ser ls *.$TIPO