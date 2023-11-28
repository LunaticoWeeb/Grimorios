# Controle de Fluxo

## Fluxo Condicional

Em Rust expressões condicionais são feitas através da palavra reservada `if` e `else`. O `if` é uma expressão que recebe uma condição e executa um bloco de código caso a condição seja verdadeira. O `else` é opcional e é executado caso a condição do `if` seja falsa.

```rust

fn main() {
      let x = 5;

      if x == 5 {
        println!("x é igual a 5");
      } else {
        println!("x não é igual a 5");
      }
}

```

Também é possível utilizar `else if` para testar mais de uma condição.

```rust

fn main() {
      let x = 5;

      if x == 5 {
        println!("x é igual a 5");
      } else if x == 6 {
        println!("x é igual a 6");
      } else {
        println!("x não é igual a 5 ou 6");
      }
}

```

As condições devem ser sempre booleanas, ou seja, `true` ou `false`. Se a condição não for booleana, o compilador irá retornar um erro.


Também é possível utilizar `if` em uma expressão `let`, porém, é necessário que os tipos de dados sejam iguais e a presença do `else` é obrigatória.

```rust

fn main() {
      let condicao = true;
      let x = if condicao {5} else {6}; // x: i32
}

```


## Fluxo de Repetição

### `loop`

Em Rust, o `loop` é uma expressão que executa um bloco de código infinitamente até que seja explicitamente interrompido.

```rust

fn main() {
      let mut counter = 0;

      loop {
            counter += 1;

            println!("counter = {}", counter);
      } // loop infinito, só é interrompido com Ctrl + C 
}

```

Para explicitamente interromper o loop, é possível utilizar a palavra reservada `break`.


```rust
fn main() {
      let mut counter = 0;

      loop {
            counter += 1;

            println!("counter = {}", counter);

            if counter == 10 {
                  break;
            }
      }
}

```

Também existe a palavra reservada `continue`, que interrompe a iteração atual e inicia a próxima iteração.

```rust

fn main() {
      let mut counter = 0;

      loop {
            counter += 1;

            if counter % 2 == 0 {
                  continue; // se o contador for par, o loop continua sem executar o código abaixo
            }

            println!("counter = {}", counter);
      }
}

```

Também é possível retornar um valor do loop, seu uso é feito através da palavra reservada `break` seguida do valor que será retornado. 

```rust

fn main() {
    let mut counter = 0;

    let result = loop { // loop pode ser utilizado numa declaração let
        counter += 1;

        if counter == 10 {
            break counter * 2; // retorna o valor 20
        }
    };

    println!("The result is {}", result);
}

```

Quando há vários loops aninhados, a palavra reservada `break` interrompe o loop mais interno. Para interromper um loop externo, é necessário utilizar a palavra reservada `break` seguida de um rótulo.

```rust

fn main() {

      let mut counter = 0;

      'outer: loop {
            counter += 2;

            println!("Entered the outer loop");

            loop {

                  println!("Counter: {}", counter);

                  println!("Entered the inner loop");

                  if counter == 10 {
                        break 'outer; // interrompe o loop externo quando counter == 10
                  } else if counter == 5 {
                        continue 'outer; // interrompe a iteração atual do loop externo
                  }

                  counter += 1;
            }
      }

      println!("Exited the outer loop");
}

```

### `while`

Para a execução de um bloco de código enquanto uma condição for verdadeira, é possível utilizar a palavra reservada `while`.

```rust

fn main() {
      let mut number = 3;     
      
      while number != 0 {
            println!("{}!", number);  
            number -= 1;
      }     

      println!("LIFTOFF!!!");
}

```

### `for`

O uso de `for` permite percorrer uma coleção de itens.

```rust

fn main() {
      let a = [10, 20, 30, 40, 50];

      for element in a {
            println!("the value is: {}", element);
      }
}

```

E com o uso de `Range` o `for` é ideal para ser utilizado para executar um bloco de código um número específico de vezes.

```rust

fn main() {
      for number in (1..4).rev() { //gera uma coleção de 1 a 3 e inverte a ordem
            println!("{}!", number);
      }

      println!("LIFTOFF!!!");
}

```
