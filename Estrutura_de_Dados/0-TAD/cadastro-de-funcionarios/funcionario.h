#ifndef funcionario_h
#define funcionario_h

#define VAZIO -1

typedef struct funcionario_t FUNCIONARIO;

FUNCIONARIO *novo_funcionario(char nome[100], char cargo[100], float salario);
void apagar_funcionario(FUNCIONARIO* funcionario);

void alterar_salario(FUNCIONARIO* funcionario, float novo_salario);
void alterar_cargo(FUNCIONARIO* funcionario, char novo_cargo[100], float novo_salario);
void alterar_nome(FUNCIONARIO* funcionario, char novo_nome[100]);

void imprimir_funcionario(FUNCIONARIO* funcionario);

#endif //FUNCIONARIO_H