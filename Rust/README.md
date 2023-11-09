# Rust

Resumo de Rust baseado no Rust Book.

## Compilador

Para verificar se a instalção foi feita corretamente digite:

```shellscript
rustc --version
```

Para atualizar o compilador:

```shellscript
rustup update
```

Para desinstalar:

```shellscript
rustup self unistall
```

Para acessar a documentação:

```shellscript
rustup doc
```

## Hello World!

```rust
fn main(){
      println("Hello, world!");
}
```

Para compilar e executar utilizamos:

```shellscript
rustc hello-world.rs && ./hello-world
```

## Cargo

Cargo é _build system_ e _package manager_ do Rust. Ele é responsável por compilar o código e instalar bibliotecas (dependências) que seu código depende, também compilando-as. Para verificar a instalçaõ do Cargo, execute:

```shellscript
cargo --version
```

Para criar um novo projeto utilizando cargo, execute:

```shellscript
cargo new project_name
```



Isto deve criar um diretório com o seguinte formato:

```shellscript
project/
├── Cargo.toml
├── .git
│   ├── config
│   ├── description
│   ├── HEAD
│   ├── hooks
│   │   └── README.sample
│   ├── info
│   │   └── exclude
│   ├── objects
│   │   ├── info
│   │   └── pack
│   └── refs
│       ├── heads
│       └── tags
├── .gitignore
└── src
    └── main.rs
```

Se neste diretório já estiver em um repositório git, ele não deve inicializar um novo repositório. Isso pode ser sobreposto com:

```shellscript
cargo new --vcs=git
```

No arquino Cargo.toml estará:

```toml
[package]
name = "hello-cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Em `[package]` fica as informações do projeto, como nome, versão, autores, licença, etc. Em `[dependencies]` ficam as dependências do projeto. 

### Compilando e executando

Para compilar o código utilizamos:

```shellscript
cargo build
```

Para compilar e executar diretamente

```shellscript
cargo run
```

E para checar se o código compila sem gerar o executável:

```shellscript
cargo check
```

O executável fica em `target/debug/`, pois estamos compilando em modo de debug. Para compilar em modo de release, que otimiza o código, utilizamos:

```shellscript
cargo build --release
```

Esse executável fica em `target/release/`, ele é otimizado para ser executado, porém é mais lento para compilar. Por isso é recomendado utilizar o modo de debug durante o desenvolvimento e o modo de release para distribuir o código.

## Fontes

[1] [Rust Book](https://rust-book.cs.brown.edu/) 
