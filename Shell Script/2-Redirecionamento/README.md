# Redirecionamento
## 5. Redirecionamento

### 5.1 *Piping*

Para redirecionar a saída de um comando para a entrada de outro, utilizamos o *pipe* `|`:

```shellscript
comando1 | comando2
```

Desta forma o *output* do `comando1` será a entrada do `comando2`.

**Exemplo**

```shellscript
ls . | grep .txt
```

### 5.2 Redirecionamento de Saída



Para redirecionar a saída do arquivo para um arquivo, utilizamos o operador `>`:

```shellscript
comando > arquivo-de-saida
```

Este operador irá reescrvier o arquivo, caso ele já exista. Para adicionar ao final do arquivo, utilizamos o operador `>>`:

```shellscript
comado1 > arquivo-de-saida
comado2 >> arquivo-de-saida
```

### 5.3 Redirecionamento de Entrada

Para redirecionar o arquivo de entrada, utilizamos o operador `<`:

```shellscript
comando < arquivo-de-entrada
```

Já caso queira redirecionar uma entrada de diversa linhas, utilizamos o operador `<<`:

```shellscript
comando << EOF
linha1
linha2
.
.
.
EOF
```

`EOF` é a palavra que delimita o fim da entrada, pode ser utilizada qualquer outra, mas por convenção é utilizada `EOF`.

Para redirecionar a entrada que seja uma *string*, utilizamos o operador `<<<`, (utiliza-se `"` `"` para delimitar a *string*):

```shellscript
comando <<< "Texto"
```