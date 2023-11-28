#include "funcionario.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>


struct funcionario_t // define dados de interesse da estrutura
{
      char nome[100];
      char cargo[100];
      float salario;
};

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