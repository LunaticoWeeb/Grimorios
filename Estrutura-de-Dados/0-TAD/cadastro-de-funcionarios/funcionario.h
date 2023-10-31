#ifndef funcionario_h
#define funcionario_h

#include <stdio.h>

typedef struct funcionario_t FUNCIONARIO;

FUNCIONARIO *novo_funcionario(char nome[100], char cargo[100], float salario);

#endif //FUNCIONARIO_H