# Resumo de C:

## Estrutura Básica:

 ```C
 #include <nome_biblioteca_nativa.h>
 #include "nome_biblioteca_externa.h"
 
 int main(){
     ...;
 }
 ```
 ### Arquivos `.h`
 Neles estão declarações de `typedef` e cabeçalhos de funções e são acompanhados de um `.c` que contém a implementação desses cabeçalhos e definições das `structs`.
 #### Sintaxe:
 ```C
 #include "arquivo.h"
 ```
 Como ele pode ser definido múltiplas vezes, utiiza-se `if` para verificar:
 ```C
 #ifndef ARQUIVO_H
    #define ARQUIVO_H
 #endif
 ```

---

## Operadores:
 ### Operadores Aritméticos:
 Operador|Função
 -|-
 `+`| adição
 `-`| subtração
 `*`| multiplicação
 `/`| divisão
 `%`| sobra da divisão

 ### Operadores de Incremento e Decremento:
 Operação|Equivalente
 -|-
 `a += b`|`a = a + b`
 `a -= b`|`a = a - b`
 `a *= b`|`a = a * b`
 `a /= b`|`a = a / b`
 `a %= b`|`a = a % b`

 ### Operadores Condicionais e Lógicos:
 Operador | Significado
 -|-
 `a == b`| `a` igual a `b`
 `a != b`| `a` diferente de `b`
 `a > b`| `a` maior que `b`
 `a >= b`| `a` maior ou igual a `b`
 `a < b`| `a` menor que `b`
 `a <= b`| `a` menor ou igual a `b`
 `a && b`| `a` AND `b`
 `a \|\| b`| `a` OR `b`
 `!a`| NOT `a`

 ### Operadores Bitwise:
 Operadores bitwise são aplicados **em cima de cada bit da variável**.

 Operador | Função
 -|-
 `a & b` | `a` AND `b`
 `a \| b` | `a` OR `b`
 `a ^ b` | `a` XOR `b`
 ` ~a` | NOT `a`
 `a << b`| desloca à esquerda bits de `a` em `b` casas 
 `a >> b`| desloca à direita bits de `a` em `b` casas

 `<<` e `>>` podem agir como multiplicadores e divisores, respectivamente:
 - `a<<b` equivalente à $a \times (2^b)$
 - `a>>b` equivalente à $\frac{a}{(2^b)}$

 E ambos não devem ser utlizados em números negativos e nem deslocar mais bits do que a variável possui.

---

## Declaração de Variáveis:
 Caractere (1 byte):
 ```C
 char nome = 'L'; //char é declarado entre aspas simples
 ```
 `String`:
 ```C
 char nome[n] = "Texto" //n recebe o número de caracteres na `string` ou vazio para o número exato de char do "Texto"
 ```
 Inteiro (4 bytes):
 ```C
 int nome = 1;
 ```
 Ponto Flutuante de Precisão Única (4 bytes):
 ```C
 float nome = 3.984235;
 ```
 Ponto Flutuante de Precisão Dupla (8 bytes):
 ```C
 double nome = 3.98423499241;
 ```

 ### Modificadores:
 `signed` força variável a ter sinal (normalmente retundante):
 ```C
 signed char nome = 'L';
 ```
 `unsigned` força a variável a ser positiva (em um bit o espaço):
 ```C
 unsigned int nome = 9128;
 ```
 `short` diminui o tamanho (2 bytes) e intervalo de um `int`:
 ```C
 short int nome = 23;
 ```
 `long` normamlmente não muda o `int`, mas usado duas vezes aumenta o tamaho para 8 bytes:
 ```C
 long long int nome = 93749081940;
 ```

---

## Constantes:
 Para definir usamos:
 ```C
 #define NOME valor
 ```
 > Nome de constante em caixa alta.

---

## Booleanos
 Como não há booleanos nativamente, é necessário definir-los:
 ```C
 #define BOOL char
 #define FALSE 0
 #define TRUE 1
 ```

---

## Impressão:
 `printf()` é a função responsável por imprimir o texto na tela.
 ```C
 printf("Texto");
 ```
 Para usar variáveis dentro da `string` do `printf()` é necessário indicar `%` e a letra correspondente pelo seu tipo.
 > **Exemplo**:
 > ```C
 > int var = 23;
 > printf("Valor de var é %d.", var);
 > ```
 > > `output`: Valor de var é 23.
 >
 > <br>

## Leitura e Entrada de Dados:
 `scanf()` é a função utlizada para entrada de dados:
 ```C
 scanf("%tipo_var", &nome_var);
 ```
 Para entrada de `char` é necessário colocar um espaço antes:
 ```C
 scanf(" %c", &var);
 ```
 **No caso de `string`s, não é necessário `&`.**
 > **Exemplo** 1:
 > ```C
 > int var;
 > scanf("%d", &var);
 > printf("Valor de var é %d.", var);
 > ```
 > > `input`: 23
 > 
 > > `output`: Valor de var é 23.
 >
 > **Exemplo** 2:
 > ```C
 > char var[20];
 > scanf("%6s", var);
 > printf("Valor de var é %d.", var);
 > ```
 > > `input`: qwertyguyqgrquygutygqtyu
 > 
 > > `output`: Valor de var é qwerty. 
 > > > O número antes do `s` indica quantos bits vão ser recebidos no `input`.
 >
 > <br>

 Também é possível usar `regex` na leitura das `strings`.
 > **Exemplo** Útil:
 > ```C
 > scanf("%[^\n]%*c", str)
 > ```
 > > Lê a `string` com espaços, até achar um `\n`. 
 >
 > <br>

 ### Tabela de Máscaras:
 |Máscara|Uso |
 |--------|----|
 |`%c`|variável `char`|
 |`%d`|variável `int`|
 |`%f`|variável `float`|
 |`%lf`|variável `double`|
 |`%.`***n***`f` &nbsp;&nbsp; ou &nbsp;&nbsp; `%.`***n***`lf`| imprime ***n*** casas do valor| 
 |`%l`***t***|variável ***t*** com `long`|
 |`%u`|variável `unsigned int`|
 |`%ll`***t***| variável ***t*** com `long long`|
 |`%s`|variável `string`|
 |`%`***n***`s` &nbsp; em &nbsp; `scanf()` |***n*** indica o número de bits que entraram na variável|
 |`&`***x***|endereço de ***x*** na memória|
 |`%p`|ponteiro (endereço da variável)|
 |`\n`|quebra linha|

---

## Conversão Explícita:
 ```C
 (tipo_de_dado)dado_convertido
 ```
 > **Exemplo**:
 > ```C
 > (int)var
 > ```
 >
 > <br>

---

## `enum`:
 `enum` é utilizado para facilitar a leitura e manutenção do código.

 ### Declaração:
 ```C
 enum var{estadoA, estadoB, estadoC}; //númera de 0 a eté o fim das chaves
 ```
 **ou**

 ```C
 enum var{estadoA = 3, estadoB = 2, estadoC = 10};
 ```

 ### Instanciação:
 ```C
 enum var objeto;
 ```

 ### Operação:
 ```C
 var = estadoB;
 ```

## Estruturas Condicionais:
 ### `if` e `else`:
 ```C
 if(condição){
     código a ser executado;
     }
 ```
 ```C
 if(condição){
     código a ser executado;
 } else {
     código a ser executado;
 }
 ```
 ```C
 if(condição){
     código a ser executado;
 } else if(condição){
     código a ser executado;
 } else {
     código a ser executado;
 }
 ```
 - As chaves só são obrigatórias se dentro de cada `if` ou `else` tiver mais de uma linah de código.
 - Obvimente podem ter tantos `else if`s necessários.
 - Também podem ter estruturas condicionais aninhadas dentro de outras.

 ### Switch-Case:
 Switcha-Case é uma estrutura condicional que pode substituir o `if` caso eles tenha que comparar uma variável com vários inteiros de uma vez.
 ```C
 switch(escolha){
     case opção1:
         bloco1;
         break;
     case opção2:
         bloco2;
         break;
     .
     .
     .
     case opçãoN:
         blocoN;
         break;
     default:
         blocoN2;
 }       
 ```
 - a comparação entre `escolha` e `opção` deve ser feita entre dois tipos inteiros (`int`, `char`, `enum`).
 - `break` evita com que caso uma das opções sejam cumpridas ele continue testando as próximas
 - `default` é executado caso nenhuma das opções sejam cumpridas.

 ### Operador Ternário:
 Forma curta de fazer estruta If-Else:
 ```C
 variável = expressão1 ? expressão2 : expressão3
 ```
 > É equivalente a:
 > ```C
 > if (expressão1){
 >   variável = expressão2;
 > } else {
 >   variável = expressão3;
 > }
 > ```
 >
 > <br>

---

## Estruturas de Repetição:
 ### Controle na Entrada:
 #### `for`:
 `for` inicia uma variável com um valor, depois checa se a condição é cumprida:
 - Se verdadeiro:
   - Executa o bloco.
   - Atualiza a variável.
 - Se não:
   - Sai do laço
 ##### Sintaxe:
 ```C
 for (inicialização; teste; atualização){
     bloco;
 }
 ```
 ##### **Exemplo**:
 ```C
 #include <stdio.h>

 int main(){
     int i, j;
     for (i = 0, j = -3; i > -3 || j < 10; i--, j+=2){
         printf("%d | %d\n", i, j);
     }
     return 0;
 }
 ```
 > `Output`:
 > ```
 > 0 | -3
 > -1 | -1
 > -2 | 1
 > -3 | 3
 > -4 | 5
 > -5 | 7
 > -6 | 9
 > ```
 >
 > <br>

 #### `while`:
 Enquanto o teste resultar em `True`, o loop será repetido.
 ##### Sintaxe:
 ```C
 while (teste) {
     bloco;
     atualização;
 }
 ```

 ### Controle na Saída:
 #### `do`:
 O `do` apenas testa depois de executar o seu bloco, logo ele é executado pelo menos uma vez.
 ##### Sintaxe:
 ```C
 do {
     bloco;
     atualização;
 } while (teste);
 ```

---

## Vetores:
 Vetor é uma coleção de itens armazenados em espaços sequênciais de memória. Esses itens podem ser acessados sozinhos pelo seu índice.
 ### Sintaxe:
 #### Declaração:
 ```C
 tipo vetor1[N];
 tipo vetor2[N] = {val1, val2, ..., valN};
 tipo vetor3[] = {val1, val2, ..., valN};
 ```
 #### Atribuição e Acesso:
 ```C
 tipo vetor[N] = {val1, val2, ..., valN};
 printf("%d", vetor[1]);
 ```
 > Output: `val2`
 ```C
 int i, vetor[10];
 for (i = 0; i < 10; i++){
    vetor[i] = i
 }
 ```
 #### Observações:
 - o vetor tem tamanho fixo
 - quando o vetor é inicializado, os espaços para cada item é preenchido por zero.
   > **Exemplo**:
   > ```C 
   >int i, vetor[4] = {1, 2};
   >for (i = 0 ; i < 4; i++){
   >    printf("%d ", vetor[i]);
   >}
   > ```
   >  > Output: `1 2 0 0` 
   >
   > <br>
 - em C o compilador não acusa erro caso você acesse um indíce que não está dentro do tamanho do vetor (*out of bounds*), ele apenas lerá o dado que está no lugar que supostamente estaria o seu item.
 ### Vetores Multidimensionais:
 #### Sintaxe:
 ```C
 tipo vetor[I][J] = {{valor1_1, valor2_1, ..., valorJ_1}, {valor1_2, valor2_2, ..., valorJ_2}, ..., {valor1_I, valor2_I, ..., valorJ_I}};
 ``` 
 > **Exemplo**:
 > ```C
 > int vetor[3][3] = {{10, 3, 5}, {1, 6, 8}, {5, 4, 2}}
 > ```
 > É como seria representado a matrix:
 > $$\vec{A} = \begin{bmatrix} 10 & 3 & 5 \\ 1 & 6 & 8 \\ 5 & 4 & 2\end{bmatrix}$$
 > <br> 
 A declaração da  matrix só pode ter o seu primeiro índice omitido:
 ```C
 int vetor[][J][K] = {{{1, 2}, {4, 5}}, {{1, 2}, {4, 5}}, {{1, 2}, {4, 5}}}
 ```

---

## `String`:
 O que diferencia uma `string` de um vetor de `char` é que a `string` termina com '\0' (`NULL`), por isso seu tamanho é o tamanho da `string` `+ 1`. Ela pode ser definida diretamente escrevendo entre `" "` (assim `NULL` também é adicionado automaticamente).
 ### Sintaxe:
 ```C
 char nome[10] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', '\0'}; //sem \0, o dado não é considerado string, e sim um vetor de caracteres.
 char nome[] = "abcdefghij"; //adiciona \0 automaticamente.
 ```

---
## Criando Funções:
 ### Sintaxe:
 ```C
 tipo nome(paramêtro1, paramêtro2, ..., paramêtroN){
    declarações;
 }
 int main(){...}
 ```
 **ou**
 ```C
 tipo nome(paramêtro1, paramêtro2, ..., paramêtroN);
 int main(){...}
 tipo nome(paramêtro1, paramêtro2, ..., paramêtroN){
    declarações;
 }
 ```
 - `tipo`: valor retornado pela função, ou `void` caso não retorne nada.
 - `paramêtro`: valores de entrada, declarados pelo seu tipo e nome e separados por vírgula.
    ```
    ...(tipo1 nome_do_parâmetro1, tipo2 nome_do_parâmetro2, ..., tipo3 nome_do_parâmetro3){...
    ```
 As funções ficam fora da `main`, e só são executados quando chamados por ela.
 #### Exemplo:
 ```C
 int add(int a, int b){
    return a + b;
 }
 ```

 ### Variável por Referência:
 A função pode também ao invés de usar uma cópia da variável recebida no seu parâmetro, ela pode alterar diretemente a variável passando sua referência (localização na memória). Para isso basta colocar um `*` antes da variável. Já quando a função for chamada é necessário passar o endereço da variável no seu parâmetro utilizando `&`.

 #### Exemplo:
 ```C
 int divide(int a, int b, int *resto){
    *resto = a%b;
    return a/b;
 }
 ```

 Já no caso de vetores é passado a primeira posição dele e seu tamanho (não funciona utalizar `sizeof` dentro da função).

  #### Exemplo:
 ```C
 #include <stdio.h>
 
 int f(int *a, int size){
     int i;
     for (i = 0; i < size; i++){
         a[i] += i;
     }
 }
 
 int main(){
     int i, vetor[4] = {1, 2, 3, 4};
     f(&vetor[0], 4);
     for (i = 0 ; i < 4; i++){
         printf("%d, ", vetor[i]);
     }
     return 0;
 }
 ```
 > `Output:` 1, 3, 5, 7,

 #### `const`:
 Em caso de parâmetros muito grandes (vetores e outras estruturas mais complexas), pode ser necessário passar a referência para ter maior eficiência. Mas, claro, depois numa futura alteração da função o programador pode acabar alterando a variável que não deveria, exemplo:
 ```C
 divisão = *a/(*b);
 ``` 
 para:
 ```C
 *a = *a/(*b);
 ```
 para evitar isso pode se utilizar `const` na declaração da variável, que gera um erro de compilação no caso da alteração dela:
 ```C
 int divide(const int *a, const int *b){
    ...
 }
 ```

 ### `void`:
 Funções tipo `void` não retornam valor nenhum. Caso ela não possua parâmetros, coloca-se `void` como parâmetro também.
 #### Exemplos:
 ```C
 void printSoma(int a, int b){
    printf("A soma de %d e %d é %d", a, b, a + b);
 }

 void printHello(void){
    printf("Hello World!");
 }
 ```

 ### `static`:
 Adicionando a palavra-chave `static` é possível restringir uso da função (que por padrão é global) apenas para o arquivo .c que ela está.
 #### Exemplo:
 ```C
 static int add(int a, int b);
 ```

---

## Ponteiros:
 Ponteiros são variáveis que guardam o endereço de outras variáveis.
 ##### Declaração:
 ```C
 tipo *nome_do_ponteiro; // o tipo é o tipo da var que ele aponta
 ```
 ##### Atribuição:
 ```C
 nome_do_ponteiro = &var;
 ```
 ##### Desferência (Acesso ao Valor no Endereço do Ponteiro):
 ```C
 var2 = *nome_do_ponteiro; // agora var2 tem o mesmo conteúdo que a var 
 ```
 ##### Incremento e Decremento de Ponteiros:
 Ponteiros podem ser incrementado e decrementados:
 ```C
 p++;
 p--;
 ```
 Isso faz com que ele se desloque um espaço de memória (o número de bytes do tamanho do tipo), logo pode se usado em vetores para mudar qual posição do vetor ele está apontando. 
 **Exemplo:**
 ```C
 int a[] = {1, 2, 3, 4, 5}, *p;
 p = &a[0];
 ```
 > p → a[0] = 1
 ```C
 p++;
 ```
 > p → a[1] = 2
 ```C
 p--;
 ```
 > p → a[0] = 1

 Podemos usar também o ponteiro para incrementar o valor dentro do vetor:
 ```C
 (*p)++;
 ```
 > p → a[0] = 2

 E fazer ambos:
 ```C
 (*p++)++;
 ```
 > a[0] = 2
 > p → a[1] = 2

 ### `void`:
 Ponteiros podem ser declarados como `void`, assim eles inicialemente não precisam ter um tipo especifíco.
 ##### Declaração:
 ```C
 void *nome_do_ponteiro;
 ```
 ##### Atribuição:
 ```C
 nome_do_ponteiro = &var;
 ```
 ##### Desferência:
 ```C
 var2 = *(tipo*)nome_do_ponteiro;
 ```
 
 ### Ponteiros null:
 Ponteiros null são ponteiros que apontam para lugar nenhum.
 ##### Declaração:
 ```C
 int *p = 0, *q = nullptr, *r = NULL;
 ```

 ### Ponteiros de Ponteiros:
 Ponteiros podem apontar para outros ponteiros.
 ##### Declaração:
 ```C
 tipo **ponteiro_de_poonteiro;
 tipo ***ponteiro_de_ponteiro_de_ponteiro;
 .
 .
 .
 tipo *...*ponteiro_de_ponteiro..._de_ponteiro;
 ```
 ##### Atribuição:
 ```C
 ponteiro_de_ponteiro = &ponteiro;
 ...
 ```
 ##### Exemplo:
 ```C
 // declaramos:
 char a, *b, **c;
 // atribuímos:
 a = 'z';
 b = &a;
 c = &b;
 ```
 > c → b → a = 'z'
 > **c = *b = a = 'z'

 #### Relação Ponteiros de Ponteiros e Vetores Multidimensionais:
 ```C
 int vetor[2][3] = {{1, 2, 3}, {4, 5, 6}};
 ```

 Ponteiros | Vetor | Valor 
 -|-|-
 `**vetor`|`vetor[0][0]`|`1`
 `*(*vetor+1)`|`vetor[0][1]`|`2`
 `*(*vetor+2)`|`vetor[0][2]`|`3`
 `**(vetor+1)`|`vetor[1][0]`|`4`
 `*(*(vetor+1)+1)`|`vetor[1][1]`|`5`
 `*(*(vetor+1)+2)`|`vetor[1][2]`|`6`

---

## Alocação Dinâmica de Memória:
 Ponteiro são usados também como receptáculos para regiões de memória alocadas dinamicamente, isso permite usar vetores, strings, matrizes, etc. de tamanhos variados, cujos tamanho não são sabidos a tempo de compilação. Para isso existem 4 funções da `<stdlib.h>`.
 ### `malloc()`:
 *Memory allocation* é usada para alocar um **único bloco de memória com tamanho especificado**. Ela retorna um ponteiro tipo `void` que pode receber *cast* para um ponteiro de qualquer forma.
 ##### Sintaxe:
 ```C
 tipo* ponteiro = (tipo*) malloc(n * sizeof(tipo));
 ```
 Assim é reservado um espaço de `n * sizeof(tipo)` para o ponteiro. **Caso não haja mais memória disponível o ponteiro retornará `NULL`.**
 ### `calloc()`:
 Esta função faz o mesmo que o `malloc()`, porém seus valores são inicializados como `0`.
 ##### Sintaxe:
 ```C
 tipo* ponteiro = (tipo*)calloc(n, sizeof(tipo));
 ```
 ### `free():`
 Depois de utilizados os ponteiros de alocação dinâmica, é necessário disponibilizar o espaço que estava reservado. Como o programa não faz isso automaticamente é necessário chamar `free()`.
 ##### Sintaxe:
 ```C
 free(ponteiro);
 ```
 Depois de chamada a região deixa de ser válida, mas o ponteiro continua apontando para ela. 

 Caso o ponteiro passado para `free()` não tiver apontando para um bloco de memória por uma das funções de alocação, isso causará um comportamento indefinido.
 ### `realloc()`:
 Para mudar o tamanho do espaço reservado para o ponteiro é utilizada a função `realloc()`, ela vai extender bloco dele ou mover o bloco para outro local caso não haja espaço.
 ##### Sintaxe:
 ```C
 ponteiro = realloc(ponteiro, n*sizeof(tipo));
 ```
 Como essa função pode alterar a locazição do espaço de memória, caso ela seja usada dentro de uma função é necessário que ela retorne este (possível) novo ponteiro.

---

## Structs:
 Structs (ou estrutura de dados) são um grupo de elementos de dados agrupados pelo mesmo nome. Esses elementos são denominados **membros** e pode ser de tipos e tamanhos diferentes.
 Eles são utilizados para representar informações complexas de uma maneira organizada.
 ##### Sintaxe:
 ```C
 struct nome_tipo{
    tipo_membro1 nome_membro1;
    tipo_membro2 nome_membro2;
    .
    .
    .
    tipo_membroN nome_membroN;
 } nomes_variáveis;
 ``` 
 - nome_tipo é o nome do tipo da `struct`.

 Assim o `nome_tipo` pode ser utilizado de forma igual a qualquer outro tipo de dado, porém sempre antecedido de `struct`. 
 Outra forma de declarar essas variáveis seria:
 ```C
 struct nome_tipo nome_variável;
 ```
 Depois de criada a variável do tipo da `struct`, para acessar os seus membros pode-se utilizar:
 ```C
 nome_variável.nome_membroI
 ```
 O conteúdo dos membros da variável do tipo da `struct` pode ser declarado diretemente:
 ```C
 struct nome_tipo nome_variável = {conteúdo_membro1, conteúdo_membro2, ..., conteúdo_membroN};
 ```
 As variáveis do tipo da `struct` podem também ser vetores:
 ```C
 struct nome_tipo nome_variável[N];
 ```
 
 ##### Exemplo:
 ```C
 struct product{
    int weight;
    double price;
 } apple, banana, melon;
 ```
 > Assim são criadas 3 variáveis: `apple`, `banana`,`melon` do tipo `product`.
 > Essas variáveis poderiam ser declaradas também assim:
 > ```C
 > struct product apple;
 > struct product banana, melo;
 > struct product orange = {20, 3.4};
 > ```
 
 ### Ponteiro de *Struct*:
 ##### Declaração e Atribuição:
 ```C
 struct nome_tipo nome_var;
 struct nome_tipo *ponteiro;
 ponteiro = &nome_var;
 ```
 ##### Acesso e Atribuição:
 ```C
 ponteiro->membro = conteúdo;
 ```
 **ou**
 ```C
 (*ponteiro).membro = conteúdo;
 ```

 ### *Structs* Aninhadas:
 ##### Declaração:
 ```C
 struct struct_original{
    tipo membro_original;
    .
    .
    .
 } var_original;

 struct struct_composta{
    struct struct_original var_original;
    tipo membro_composta;
    .
    .
    .
 } var_composta;
 ```
 ##### Acesso e Atribuição:
 ```C
 var_original.membro_original = conteúdo_original;
 var_composta.var_original = var_original;
 ```

 ##### Exemplo:
 ```C
 #include <stdio.h>
 #include <string.h>
 
 struct manga_t{
     char title[40];
     int year;
 } manga;
 
 struct anime_t{
     struct manga_t manga;
     char studio[30];
     int year;
 } anime;
 
 void printManga (struct manga_t manga);
 void printAnime (struct anime_t anime);
 
 int main(){
     strcpy(manga.title, "Kono Subarashii Sekai ni Shufuku o!");
     manga.year = 2012;
 
     strcpy(anime.studio, "Studio Deen");
     anime.year = 2016;
 
     anime.manga = manga;
     printAnime(anime);
     return 0;
 }
  
 void printManga(struct manga_t manga){
     printf("\nManga Title:\n%s\n", manga.title);
     printf("\nManga Year of Release: \n%d\n", manga.year);
 }
 
 void printAnime(struct anime_t anime){
     printManga(anime.manga);
 
     printf("\nAnime Studio:\n%s\n", anime.studio);
     printf("\nAnime Year of Release: \n%d\n", anime.year);
 }
 ```
 > **Output:**
 > Manga Title:
 > Kono Subarashii Sekai ni Shufuku o!
 > 
 > Manga Year of Release: 
 > 2012
 > 
 > Anime Studio:
 > Studio Deen
 > 
 > Anime Year of Release: 
 > 2016

 ### Alocação Dinâmica:
 Para realizar alocação dinâmica com a struct é necessário apenas colocar o tipo como `struct nome_tipo`, por **exemplo:**
 ```C
 struct manga_t *pmanga;

 pmanga = (struct manga_t*) malloc(sizeof(struct manga_t)*1);
 ```

---

## Criando Tipos:
 É possível criar novos tipos que funcionam igual a `int`, `char`, etc.
 ### Sintaxe:
 ```C
 typedef tipo nome_do_tipo;
 ```
 **ou para `structs`**
 ```C
 typedef struct{
    tipo membro;
    .
    .
    .
 } nome_tipo;
 ```

---

## Bibliotecas e Funções:
 - `<stdio.h>`:
   - `sizeof(x)` retorna o tamanho de `x`.
   - `strlen(x)` retorna tamanho da `string` `x` (não funciona em vetores de `char`), é mais adequado que `sizeof()` pois não conta o `NULL`. 
 - `<stdlib.h>`:
   - `rand()` retorna um número aleatório no intervalo criado.
     > **Exemplo**:
     > ```C
     > var = rand() % 100; //gera uma valor entre 0 e 99
     > var2 = rand() % 100 + 1; //gera um valor entre 1 e 100
     > var3 = rand() % 30 + 1985; //gera um valor entre 1985-2014
     > ```
     > <br>
   - `abs()` retorna módulo inteiro.
 - `<math.h>`:
   - `fabs()` retorna módulo float.
 - `<string.h>`:
   - `strcpy(str_destino, str_origem)` copia o conteúdo da `str_origem` para a `str_destino` e retorna a `string` copiada.
