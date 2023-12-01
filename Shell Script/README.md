# Shell Script


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

## Fontes
[1] Vídeo: [freeCodeCamp](https://youtu.be/tK9Oc6AEnR4)
