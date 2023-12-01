# Shell Script

Anotações sobre o uso de shell script.

## Executando um script

Normalmente os shellscripts iniciam com o _shebang_ `#!/bin/bash`, isso sinaliza pro sistema que para utilizar o interpretador `bash` que geralmente está localizado no diratório `bin/`. Porém como isso não se aplica a todo sistema que utilize Bash, uma forma mais portável do código é com `#!/usr/bin/env bash`, isso sinaliza pro sistema criar um ambiente para executar o código e utilizar o interpretador Bash. Dessa forma temos ([programa](./Programas/hello-world.sh)):

```bash
#!/usr/bin/env bash
echo "Hello, World!" # Imprime Hello, World! 
```

> - `echo` imprime o conteúdo.
> - `#` comenta o código.

Para executar é necessário dar permissão de execução ao script, para isso utilizamos o comando `chmod` que muda as permissões do arquivo e `+x` que
adiciona (`+`) a permissão de execução (`x`).

```bash
chmod +x script.sh
```

Assim podemos executar:

```bash
./script.sh
```

Outra forma de executar é chamando diretamento o interpretador, assim não é necessário adicionar o _shebang_:

```bash
bash script.sh
```

## Fontes

[1] Vídeo: [freeCodeCamp](https://youtu.be/tK9Oc6AEnR4)
