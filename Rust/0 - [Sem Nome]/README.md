Impressão

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

```rust

```
