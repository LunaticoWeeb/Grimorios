# Exemplo de TAD: Cadastro de Funcionários

Para implementar o cadastro de funcionários utilizando TAD foi utilizado três arquivos:

- `main.c`: onde `funcionario.h` será utilizado.
- `funcionario.h`: _header_ do `funcionario.c` com definições e declarações.
- `funcionario.c`: _source file_ onde estão implementadas a estrutura e funções utilizadas.

## TAD `FUNCIONARIO`

### _Header_

Primeiro é necessário utilizar o _include guard_ para impedir que ocorra uma inclusão dupla:

```c
#ifndef funcionario_h
#define funcionario_h

...

#endif //FUNCIONARIO_H
```

No _header_ ocorrerão as definições de constantes como `VAZIO` que vai ser utilizado posteriormente, a definição do tipo de dado `FUNCIONARIO`, as inclusões de bibliotecas, claro, a declaração das funções do TAD. 

```c
#ifndef funcionario_h
#define funcionario_h

#define VAZIO -1

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct funcionario_t FUNCIONARIO;

FUNCIONARIO *novo_funcionario(char nome[100], char cargo[100], float salario);
void apagar_funcionario(FUNCIONARIO* funcionario);

void alterar_salario(FUNCIONARIO* funcionario, float novo_salario);
void alterar_cargo(FUNCIONARIO* funcionario, char novo_cargo[100], float novo_salario);
void alterar_nome(FUNCIONARIO* funcionario, char novo_nome[100]);

void imprimir_funcionario(FUNCIONARIO* funcionario);

#endif //FUNCIONARIO_H
```

Como as declarações e definições estão separadas do corpo do código, fica mais fácil de modificar sua implementação explicitamente afetar o seu uso pro usuário.

### _Source File_

No _source file_  que serão implementadas as funções do TAD, ele apenas conterá o próprio _header_ como inclusão. Na compilação `funcionario.h` e `funcionario.c` serão tratados como um programa único.

```c
#include "funcionario.h"
```

Como dito, o tipo de dado abstrato é separado na sua estrutura que possui os dados de interesses:

```c
struct funcionario_t // define dados de interesse da estrutura
{
      char nome[100];
      char cargo[100];
      float salario;
};
```

E as funçẽos que vão manipular os dados:

```c
FUNCIONARIO *novo_funcionario(char nome[100], char cargo[100], float salario)
{
      // Alocando o espaço da estrutura:
      FUNCIONARIO *novo_funcionario = (FUNCIONARIO*) malloc(sizeof(FUNCIONARIO));

      // Atribue os dados:
      strcpy(novo_funcionario->nome, nome); 
      strcpy(novo_funcionario->cargo, cargo);
      novo_funcionario->salario = salario;
}

void apagar_funcionario(FUNCIONARIO* funcionario){
      free(funcionario);
}

void alterar_salario(FUNCIONARIO* funcionario, float novo_salario)
{
      funcionario->salario = novo_salario;
}

void alterar_cargo(FUNCIONARIO* funcionario, char novo_cargo[100], float novo_salario)
{
      strcpy(funcionario->cargo, novo_cargo);
      
      if (novo_salario != VAZIO){ 
            alterar_salario(funcionario, novo_salario);
      };
}

void alterar_nome(FUNCIONARIO* funcionario, char novo_nome[100])
{
      strcpy(funcionario->nome, novo_nome);
}

void imprimir_funcionario(FUNCIONARIO* funcionario)
{
      printf("Nome: %s", funcionario->nome);
      printf("\nCargo: %s", funcionario->cargo);
      printf("\nSalário: %.2f\n\n", funcionario->salario);
}
```

## Uso do TAD

Nesse caso o uso do TAD foi utilizado diretamente na função `main`, nele toda a manipulação dos dados serão feitas através das funções definidas, logo de forma indireta, o que permite que seja mais seguro de utilizar:

```c
#include <stdio.h>
#include "funcionario.h"

int main(void){

      char nome1[100] = "Josué";
      char cargo1[100] = "Diretor";
      float salario = 8000;

      FUNCIONARIO *funcionario1 = novo_funcionario(nome1, cargo1, salario);

      imprimir_funcionario(funcionario1);

      char cargo1b[100] = "Vice-diretor";

      alterar_cargo(funcionario1, cargo1b, VAZIO);

      imprimir_funcionario(funcionario1);

      char cargo1c[100] = "Secretário";

      alterar_cargo(funcionario1, cargo1c, 3500);

      imprimir_funcionario(funcionario1);

      char nome2[100] = "Enoque";
      char cargo2[100] = "Porta-voz";
      float salario2 = 2000;

      FUNCIONARIO *funcionario2 = novo_funcionario(nome2, cargo2, salario2);

      imprimir_funcionario(funcionario1);
      imprimir_funcionario(funcionario2);

      alterar_salario(funcionario2, 500000);

      imprimir_funcionario(funcionario2);
      
      char nome2a[100] = "Metatron";
      alterar_nome(funcionario2, nome2a);

      imprimir_funcionario(funcionario2);

      apagar_funcionario(funcionario1);

      return 0;
}
```
