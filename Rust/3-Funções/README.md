# Funções


Em Rust, funções são declaradas com a palavra-chave `fn` seguida do nome da função, uma lista de parâmetros entre parênteses e o tipo de retorno da função. O corpo da função é delimitado por chaves `{}`. A convenção de nomeação de funções em Rust é `snake_case`.

```rust
fn hello_world() {
    println!("Hello, world!");
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn main() {
      hello_world();
      print_number(5);
}

```
Funções com multíplos parâmetros são separados por vírgulas.

```rust

fn print_numbers(x: i32, y: i32) {
    println!("x is: {}", x);
    println!("y is: {}", y);
}

```

## Expressões e declarações

Em Rust, é importante a distinção entre expressões e declarações. Expressões são avaliadas e retornam um valor, enquanto declarações não. Por exemplo, uma atribuição é uma declaração, enquanto uma função que retorna um valor é uma expressão.

- **Expressões**: avaliadas e retornam um valor
- **Declarações**: não avaliadas e não retornam um valor


```rust

fn add_one(x: i32) -> i32 {
    x + 1
} // Expressão

fn main() {
    let x = 5; // Declaração
    let y = {
        let x_squared = x * x; // Declaração
        let x_cube = x_squared * x; // Declaração

        x_cube + x_squared + x // Expressão -> retorna o valor que será atribuído a y
    };
    
    let z = add_one(y); // Expressão
}

```

Declarações terminam com um `;`, enquanto expressões não. Por exemplo, `x + 1` é uma expressão, enquanto `let x = 5;` é uma declaração.

## Retorno de funções

Funções em Rust podem retornar valores de forma explícita com a palavra-chave `return` ou de forma implícita com a última expressão da função.

```rust

// função com retorno explícito
fn foo(x: i32) -> i32 {
    return x;
}

// função com retorno implícito
fn bar(x: i32) -> i32 {
    x
}

```

Toda função que retorna valores precisam ter o tipo de retorno especificado após a lista de parâmetros com a sintaxe `-> <tipo>`.
