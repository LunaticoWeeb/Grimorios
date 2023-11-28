# Estrutura de Dados

Resumo sobre estrutura de dados implementadas em C.

## Organização

Cada diretório contém um arquivo `README.md` com uma breve explicação sobre a estrutura de dados e um diretório `programa` com a implementação da mesma. A ideia é fazer uma breve explicação da estrutura, implementar em um programa e destrinchar sua implementação no `programa/README.md`.

Assim o modelo fica:

```plaintext
n-estrutura-de-dados  
├── programa  
│   ├── compilacao.sh  
│   ├── estruturas  
│   │   ├── esutura-1.c  
│   │   └── estrutura-2.c  
│   ├── estrutura-1.h  
│   ├── estrutura-2.h  
│   └── main.c  
└── README.md  
```

A produção de cada estrutura é feita na seguinte ordem:

1. Explicação da estrutura de dados no `README.md`
2. Implementação da estrutura de dados no diretório `estruturas`
3. Implementação de um programa que utiliza a estrutura de dados no diretório `programa`
4. Explicação da implementação da estrutura de dados no `README.md` do diretório `programa`
