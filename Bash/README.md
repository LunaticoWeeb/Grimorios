# Bash

## 1. Hello World

```shellscript
#!/usr/bin/env bash
echo "Hello, World!"
```

> - `#!/usr/bin/env bash` busca o interpretador bash no `PATH` do sistema e executa. Isso permite que o script seja executado em qualquer sistema operacional Linux com bash instalado. Uma alternativa é usar `#!/bin/bash`, que é o caminho padrão do bash no Linux, mas não é garantido que seja o mesmo em todos os sistemas operacionais.
> - `echo` imprime o conteúdo.

Para executar é necessário dar permissão de execução ao script:

```shellscript
chmod +x script.sh
```

Para finalmente executar:

```shellscript
./script.sh
```

Outra forma de executar é:

```shellscript
bash script.sh
```

> Neste caso não é necessário dar permissão de execução ao script e nem informar o interpretador bash.

## 2. Variáveis

Definindo variáveis:

```shellscript
VAR=conteudo
```

Acessando a variável:

```shellscript
$VAR
```

## 3. Entrada

Para receber um *input* do usuário:

```shellscript
read ENTRADA
```

Assim o *input* escrito em seguida pelo usuário deve ser armazenado na variável `ENTRADA`.

## 4. Argumentos Posicionais

Para receber argumentos para o programa em sua execução, podemos utilizar argumentos posicionais, por exemplo, se temos um script que recebe dois argumentos do usuário, podemos já defini-los antes de ser interpretado:

```shellscript
./programa.sh argumento1 argumento2
```

Para isso utilizamos:

```shellscript
$1 $2
```


**Exemplos:**

```shellscript
echo $1 e $2
```

```shellscript
VAR1=$1
VAR2=$2

echo $1 e $2
```

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

## 8. Estruturas de Repetição

### 8.1 Arrays

```shellscript
LISTA=(valor1 valor2 valor3)

echo ${LISTA[0]} # valor1

echo ${LISTA[1]} # valor2

echo ${LISTA[2]} # valor3

echo ${LISTA[@]} # valor1 valor2 valor3
```

### 8.2 For

## Tabelas

### Tabela de Comandos:

**COMANDO** | **EFEITO**
-|-
`man` *comando* | mostra manual do comando
`cd` *diretório* | vai pro *diretório*
`ls` *diretório* | lista arquivos e diretórios do *diretório*
`echo` *conteúdo* | imprime *conteúdo*
`cat` *arquivo* | mostra conteúdo do *arquivo*
`touch` *arquivo* | cria *arquivo*
`mkdir` *diretório* | cria *diretório*
`rm` *arquivo* | remove *arquivo*
`rmdir` *diretório* | remove *diretório*
`cp` *arquivo* *diretório* | copia *arquivo* para *diretório*
`mv` *arquivo* *diretório* | move (ou renomeia) *arquivo* para *diretório*
`wc` *arquivo* | conta linhas, palavras e caracteres do *arquivo*
`grep` *padrão* *arquivo* | mostra linhas do *arquivo* que contém *padrão*

### Tabela de Símbolos:

**SIMBOLO** | **EFEITO**
-|-
`~` | diretório home
`.` | diretório atual
`..` | diretório pai
`*` | qualquer coisa
`?` | um caractere
`$?` | código de saída


## Tabela de Operadores:

**OPERADOR** | **EFEITO**
-|-

