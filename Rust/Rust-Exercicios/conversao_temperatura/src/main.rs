use std::io;

fn main() {

    let mut escala_entrada: f32;
    let mut escala_saida: f32;
    let mut temperatura_entrada: f32;
    let opcoes_validas = [1.0, 2.0, 3.0];

    loop{        
        println!("\nEscreva a temperatura de entrada:\n 1 - Celsius\n 2 - Fahrenheit\n 3 - Kelvin");

        escala_entrada = entra_valor();

        // teste opção válida
        if !checa_opcao(escala_entrada, opcoes_validas){
            println!("\nPor favor digite uma opção válida!");
            continue;
        }

        println!("\nEscreva a temperatura de saída:\n 1 - Celsius\n 2 - Fahrenheit\n 3 - Kelvin");

        escala_saida = entra_valor();
        
        // teste opção válida
        if !checa_opcao(escala_saida, opcoes_validas){
            println!("\nPor favor digite uma opção válida!");
            continue;
        }

        println!("\nEscreva o valor da temperatura:");
        temperatura_entrada = entra_valor();

        let temperatura_saida = conversao_temp(escala_entrada, escala_saida, temperatura_entrada);
        println!("\nTemperatura {temperatura_saida}\n");

        println!("Deseja sair?\n 1 - Sim\n 2 - Não");
        let encerrar = entra_valor();

        if encerrar == 1. {break};
    }
    
}

// Interação com usuário
fn entra_valor() -> f32 {
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Falhou em ler a linha.");
    
    let input: f32 = input.trim().parse()
        .expect("Falhou em converter String -> f32");

    input
}

fn checa_opcao(opcao: f32, opcoes_validas: [f32; 3]) -> bool {
    
    let mut validade: bool = false;
    
    for x in opcoes_validas{
        validade = if x == opcao {true} else {continue};
    }

    validade
}

//Conversão de temperatura:
fn celsius_para_fahrenheit(temp_celsius: f32) -> f32 {
    let temp_fahrenheit = temp_celsius * (9./5.) + 32.;
    temp_fahrenheit
}

fn farenheit_para_celsius(temp_fahrenheit: f32) -> f32 {
    let temp_celsius = (temp_fahrenheit - 32.) * (5./9.);
    temp_celsius
}

fn celsius_para_kelvin(temp_celsius: f32) -> f32 {
    let temp_kelvin = temp_celsius + 273.15;
    temp_kelvin
}

fn kelvin_para_celsius(temp_kelvin: f32) -> f32 {
    let temp_celsius = temp_kelvin - 273.15;
    temp_celsius
}

fn farenheit_para_kelvin(temp_fahrenheit: f32) -> f32 {
    let temp_kelvin = farenheit_para_celsius(temp_fahrenheit) + 273.15;
    temp_kelvin
}

fn kelvin_para_fahrenheit(temp_kelvin: f32) -> f32{
    let temp_fahrenheit = celsius_para_fahrenheit(temp_kelvin - 273.15);
    temp_fahrenheit
}

fn temperatura_impossivel(opcao_saida: f32, temperatura_saida: f32) -> bool{
    let temp_impossivel = [-273.15, -459.67, 0.];

    let op = if opcao_saida == 1. {0} else if opcao_saida == 2. {1} else {2};
 
    if temperatura_saida < temp_impossivel[op] {
        true
    } else {false}
}

fn conversao_temp(entrada: f32, saida: f32, temperatura_entrada: f32) -> f32 {

    let temperatura_saida: f32;

    if entrada == 1.0 && saida == 2.0 { // C -> F

        temperatura_saida = celsius_para_fahrenheit(temperatura_entrada);

    } else if entrada == 2.0 && saida == 1.0 { // F -> C

        temperatura_saida = farenheit_para_celsius(temperatura_entrada);

    } else if entrada == 1.0 && saida == 3.0 { // C -> K

        temperatura_saida = celsius_para_kelvin(temperatura_entrada);

    } else if entrada == 3.0 && saida == 1.0 { // K -> C

        temperatura_saida = kelvin_para_celsius(temperatura_entrada);

    } else if entrada == 2.0 && saida == 3.0 { // F -> K

        temperatura_saida = farenheit_para_kelvin(temperatura_entrada);

    } else if entrada == 3.0 && saida == 2.0 { // K -> F

        temperatura_saida = kelvin_para_fahrenheit(temperatura_entrada);

    } else { temperatura_saida = -32_000. } // Erro 

    if temperatura_impossivel(saida, temperatura_saida) {
        println!("\nTemperatura impossível!\n");
    }

    temperatura_saida
    
}

