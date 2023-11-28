# Tipos de Dados

Apesar do compilador inferir o tipo das variáveis, Rust é uma linguagem estaticamente tipada, ou seja, o tipo de todas as variáveis deve ser conhecido em tempo de compilação. O tipo de uma variável pode ser definido explicitamente ou implicitamente.

## Tipos Escalares

### Inteiros

Os inteiros em Rust são dividos por tamanho (de 8 a 128 bits) e sinal (positivo ou negativo). O tipo padrão é `i32` (inteiro de 32 bits com sinal). Os inteiros sem sinal são representados pelo prefixo `u` (ex: `u8`).

Tamanho | Com sinal | Sem sinal
------- | --------- | ---------
8 bits  | `i8`      | `u8`
16 bits | `i16`     | `u16`
32 bits | `i32`     | `u32`
64 bits | `i64`     | `u64`
128 bits| `i128`    | `u128`
arquitetura | `isize` | `usize`



**Nota**: O tipo `isize` e `usize` é o tamanho do ponteiro da arquitetura do computador. Por exemplo, em um computador de 64 bits, o tipo `isize` é de 64 bits.



A declaração explícita de uma variável é feita da seguinte forma:

```rust
let x: i32 = 5;
```

O Rust permite que números literais sejam escrito com `_` para melhorar a legibilidade. Por exemplo, o número `1000000` pode ser escrito como `1_000_000`. O compilador irá ignorar os `_`.

Ele também permite que números literais sejam escritos em hexadecimal, octal e binário. Para isso, basta adicionar um prefixo ao número:

Numeros | Prefixo | Exemplo
------- | ------- | -------
Decimal | Nenhum  | `98_222`
Hexadecimal | `0x` | `0xff`
Octal | `0o` | `0o77`
Binário | `0b` | `0b1111_0000`
Byte (u8 apenas) | `b` | `b'A'`

### Ponto Flutuante

Os números de ponto flutuante são representados pelo tipo `f32` (32 bits) e `f64` (64 bits). O tipo padrão é `f64`.

```rust

let x = 2.0; // f64

let y: f32 = 3.0; // f32
```

> **Nota**: Os números de ponto flutuante seguem o padrão IEEE-754.

#### Operações

Rust suporta as operações aritméticas básicas adição, subtração, multiplicação, divisão e resto. Elas podem ser feitas com os operadores `+`, `-`, `*`, `/` e `%` respectivamente. No caso da divisão de inteiros, o resultado será o inteiro mais próximo.

```rust

// adição
let sum = 5 + 10; // 15

// subtração
let difference = 95.5 - 4.3; // 91.2

// multiplicação
let product = 4 * 30; // 120

// divisão
let quotient = 56.7 / 32.2; // 1.7608695652173911
let truncated = 2 / 3; // 0

// resto
let remainder = 43 % 5; // 3
```

### Booleano

Rust possui o tipo booleano `bool` que pode assumir os valores `true` e `false`.

```rust
let t = true;

let f: bool = false;
```

### Caractere

O tipo `char` representa um caractere que ocupa 4 bytes de memória e representa um caractere Unicode Scalar Value (variando de `U+0000` a `U+D7FF` e de `U+E000` a `U+10FFFF`). Os caracteres são escritos entre aspas simples.

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

## Tipos Compostos

Tipos compostos são tipos que podem agrupar vários valores em um único tipo. Rust possui dois tipos compostos: tuplas e arrays. Ambas são tipos de tamanho fixo, ou seja, o tamanho delas deve ser conhecido em tempo de compilação. Para tipos de tamanho variável, Rust possui os tipos `String` e `Vec<T>`.

### Tuplas

As tuplas são um tipo de tamanho fixo que podem agrupar vários valores de tipos diferentes. Elas são declaradas entre parênteses e os valores são separados por vírgula.

```rust

let tup: (i32, f64, u8) = (500, 6.4, 1); // tupla com 3 elementos com declaração explícita

let tup = (500, 6.4, 1); // tupla com 3 elementos com declaração implícita

```

Para acessar os elementos de uma tupla, podemos usar a notação de ponto `.` seguida do índice do elemento.

```rust

let tup = (500, 6.4, 1);

let x = tup.0; // 500
let y = tup.1; // 6.4
let z = tup.2; // 1

```

Outra forma de acessar os elementos de uma tupla é através de _destructuring_.

```rust

let tup = (500, 6.4, 1);

let (x, y, z) = tup;

println!("O valor de y é: {}", y); // O valor de y é: 6.4

```

Um tipo de tupla especial é a tupla vazia `()`. Ela é usada quando não há nenhum valor para ser retornado. Por exemplo, uma função que não retorna nada, retorna `()`. Ela é chamada de _unit type_.

### Arrays

Um _array_ é um tipo de tamanho fixo que pode agrupar vários valores do mesmo tipo. Os arrays são declarados com colchetes e os valores são separados por vírgula.

```rust

let a = [1, 2, 3, 4, 5]; // array com 5 elementos com declaração implícita

let a: [i32; 5] = [1, 2, 3, 4, 5]; // array com 5 elementos com declaração explícita

```

Elas podem ser declaradas com o mesmo valor para todos os elementos com:

```rust

let a = [3; 5]; // [3, 3, 3, 3, 3]

```

Para acessar os elementos de um array, podemos usar a notação de colchetes `[]` seguida do índice do elemento.

```rust

let a = [1, 2, 3, 4, 5];

let x = a[0]; // 1

```

> **Nota**: O índice do primeiro elemento de um array é 0.
