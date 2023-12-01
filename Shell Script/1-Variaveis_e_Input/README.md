# Variáveis e _Input_

## Variáveis

Definindo variáveis:

```shellscript
VAR=conteudo
```

Acessando a variável:

```shellscript
$VAR
```

> [Exemplo](./Programas/variaveis.sh)

## _Input_

Para receber um *input* do usuário:

```shellscript
read ENTRADA
```

Assim o *input* escrito em seguida pelo usuário deve ser armazenado na variável `ENTRADA` ([Exemplo](./Programas/entrada.sh)).

## Argumentos Posicionais

Para receber argumentos para o programa em sua execução, podemos utilizar argumentos posicionais, por exemplo, se temos um script que recebe dois argumentos do usuário, podemos já defini-los antes de ser interpretado:

```shellscript
./programa.sh argumento1 argumento2
```

Para isso utilizamos:

```shellscript
$1 $2
```

**Exemplos**

```shellscript
echo $1 e $2
```

```shellscript
VAR1=$1
VAR2=$2

echo $1 e $2
```

> [Exemplo](./Programas/arg-posicionais.sh)
