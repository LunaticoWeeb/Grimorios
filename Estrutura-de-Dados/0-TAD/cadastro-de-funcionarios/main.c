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

      imprimir_funcionario(funcionario1);

      return 0;
}