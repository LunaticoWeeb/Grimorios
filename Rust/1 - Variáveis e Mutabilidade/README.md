# Variáveis e Mutabilidade

## Mutabilidade

Para declarar uma variável em Rust utilizamos:

```rust
let x = 1;
```

Rust, por _default_, não permite que variáveis sejam mutáveis, ou seja, que seu valor seja alterado após a sua declaração. Isso dificulta erros de programação, pois o compilador não permite que o programador altere o valor de uma variável sem querer.



Caso seja necessário que uma variável seja mutável, é necessário utilizar a palavra-chave `mut` antes do nome da variável.

```rust
let mut y = 2;
y = 3;
```

`let` só pode ser usado dentro do escopo de uma função, ou seja, não pode ser usado em escopo global.

## Constantes

Constantes são variáveis que não podem ser alteradas, ou seja, nunca são mutáveis. Para declarar uma constante, utilizamos a palavra-chave `const` antes do nome da variável e o tipo da variável deve ser declarado explicitamente.

```rust
const TWO: f32 = 1 + 1; // contantes podem ser declaradas em forma de expressões
```

Por convenção, o nome de constantes é escrito em letras maiúsculas e com _underscores_ separando as palavras. Por exemplo, `MAX_POINTS`.

Ao contrário de variáveis, constantes podem ser declaradas em escopo global. Porém, elas só podem ser declaradas com valores constantes, ou seja, **não podem ser definidas em tempo de execução**.

## _Shadowing_

Apesar de não ser possível alterar o valor de uma variável imutável, é possível declarar uma nova variável com o mesmo nome da variável anterior. Isso é chamado de _shadowing_.

```rust

let x = 5;
let x = x + 1;
let x = x * 2;

println!("O valor de x é: {}", x); // O valor de x é: 12
```

A vantagem do _shadowing_ é que podemos alterar o tipo da variável. Por exemplo:

```rust
let spaces = "   ";
let spaces = spaces.len();// error[E0308]: mismatched types
```

Ao contrário do _shadowing_, **se tentarmos alterar o tipo de uma variável mutável, o compilador irá retornar um erro.** Para alterar o tipo de uma variável mutável, é necessário também utilizar _shadowing_:

```rust

let mut spaces = "   ";
let mut spaces = spaces.len(); // Ok
```

_**Obs.:**_ _Shadowing_ não é o mesmo que mutabilidade. Quando utilizamos _shadowing_, estamos declarando uma nova variável com o mesmo nome da variável anterior. Já quando utilizamos mutabilidade, estamos alterando o valor da variável.

Rust também permite que utilizando _shadowing_ seja possível alterar o valor de uma variável de forma delimitada por escopo. Por exemplo:


```rust

let x = 5;
let x = x + 2;

{
    let x = x * 2;
    println!("O valor de x no escopo interno é: {}", x); // O valor de x no escopo interno é: 14
}

println!("O valor de x no escopo externo é: {}", x); // O valor de x no escopo externo é: 7
```

