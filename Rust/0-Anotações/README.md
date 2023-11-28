# Anotações

**Anotações avulsas que deve ser organizadas posteriormente ou incluídas em outros módulos.**

## Conveções

- Nome de funções e variáveis: snake_case
- Nome de constantes: SCREAMING_SNAKE_CASE

## Comentários

Comentários são escritos após `//`.

## Impressão

```rust
fn main() {
      let x = 5;
      let y = 10;
  
      // Imprimir uma string simples
      println!("Hello, world!");
  
      // Imprimir uma variável
      println!("{}", x);
  
      // Imprimir com formatação
      println!("x is {}", x);
  
      // Imprimir com argumentos de variáveis
      println!("x is {} and y is {}", x, y);
  
      // Imprimir com argumentos posicionais
      println!("x is {0} and y is {1}", x, y);
      println!("y is {1} and x is {0}", x, y);
  
      // Imprimir com argumentos nomeados
      println!("x is {x} and y is {y}", x=x, y=y);
}
```

## Strings e Entrada de Dados

Para dados serem inseridos pelo usuário é necessário utilizar o tipo `String` que pode ser criado com o método `String::new()`.

```rust
let mut input = String::new();
```

A `String` criada assim não possui um tamanho definido, então é possível adicionar caracteres a ela com o método `push_str()`.

```rust
input.push_str("Hello");
```

Já para no caso de entrada de dados pelo usuário utilizamos a função `std::io::stdin()`

```rust
use std::io;

let mut input = String::new();

io::stdin()
      .read_line(&mut input) // &mut input é uma referência mutável para a String input
      .expect("Failed to read line"); // Caso ocorra algum erro, a mensagem será exibida
```

## Conversão de String 

Quando se recebe dados do usuário, eles são recebidos como `String`, mas para realizar operações matemáticas é necessário que eles sejam convertidos para um tipo numérico. Para isso, utilizamos o método `parse()`.

```rust

let input = String::new();

let input: i32 = input // Converte a String input para i32
      .trim() // Remove espaços em branco
      .parse() // Converte para o tipo especificado
      .expect("Failed to convert"); // Caso ocorra algum erro, a mensagem será exibida
```
