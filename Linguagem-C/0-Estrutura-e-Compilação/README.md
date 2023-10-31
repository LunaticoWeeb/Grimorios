# Estrutura Básica


Um programa `.c` é geralmente dividido em 6 partes:

1. Inclusão de bibliotecas
2. Definição de constantes
3. Declaração das funções
4. Função `main`
5. Definição das funções

```c
// Inclusão das bibliotecas:
#include <biblioteca-padrão.h>
#include "caminho/biblioteca-personalizada.h"

// Definição de constantes:
#define CONSTANTE 1

// Definição de estruturas e tipos:
struct estrutura1 {
      /*
            ...
      */
}

// Declaração de funções:
void funcao(int parametro1, char parametro2);

// Função principal:
int main(void) {

      /*
            ...
      */

      return 0;
}

// Definição de funções:
void funcao(int parametro1, char parametro2) {
      /*
            ...
      */
}
```

# Modularização

Para programas mais complexos, pode-se modularizar separando funçẽos, estruturas e constantes em arquivos que podem sere reutilizados em outros programas. Para isso, separamos o programa em 3 arquivos:

1. `main.c`: contém a função `main` e as declarações das funções
2. `funcoes.h`: contém as declarações das funções
3. `funcoes.c`: contém as definições das funções


