# Estruturas_Condicionais

## 6. Estruturas Condicionais

### 6.1 If, Else e Elif



```shellscript
if [ string1 = string2 ] then
    comando1
    comando2
elif [ numero1 -eq numero2 ] then
    comando3
    comando4
else
    comando5
    comando6
fi
```

> - `[]` é um comando que retorna `0` se a condição for verdadeira e `1` se for falsa.
> - `=` é um operador de igualdade para *strings*.
> - `-eq` é um operador de igualdade para números.

### 6.2 Case

```shellscript
case $VAR in
    valor1 | valor2)
        comando1
        comando2
        ;;
    valor3)
        comando3
        comando4
        ;;
    *)
        comando5
        comando6
        ;;
esac
```

> - `|` é o operador *or*.
> - `;;` é o delimitador de cada *case*.
> - `*)` é o *default*.



