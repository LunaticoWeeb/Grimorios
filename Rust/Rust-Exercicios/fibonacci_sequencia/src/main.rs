use std::io;

fn main() {

    println!("Digite até qual termo você que gerar:");
    let termo_max = entra_n();

    let enesimo_termo = seq_fibonacci(termo_max);

    println!("O termo {termo_max} é o {enesimo_termo}");
}

fn entra_n() -> u32 {
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falhou em ler linha.");

    let input: u32 = input.trim().parse()
        .expect("Falhou em converter String -> u32");

    input
}

fn seq_fibonacci(enesimo: u32) -> u32{

    let mut penultimo_gerado = 0;
    let mut ultimo_gerado = 1;
    let mut novo_termo = 0;

    if enesimo > 2{
        
        for _n in 2..enesimo {

            novo_termo = penultimo_gerado + ultimo_gerado;
            penultimo_gerado = ultimo_gerado;
            ultimo_gerado = novo_termo;
        }

    } else {
        novo_termo = if enesimo == 2 {1} else {0};
    }

    novo_termo
}