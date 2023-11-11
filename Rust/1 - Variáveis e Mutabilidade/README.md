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
