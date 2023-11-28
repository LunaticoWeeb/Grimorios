use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
    use:  uitliza livraria que não está presente no prelúdio
    - prelúdio: conjunto de items automaticamente importados, como println!()
    std: standard library
    io: part of the std that deal if input and output
    rand: random library
    Rng: define métodos de geração de números aleatórios
    cmp: compara dois valores
*/

fn main() {

    println!("Adivinhe o número!"); //print line (! é pq não é uma função e sim um macro)

    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*
        var imutável
        rand::thread_rng() -> função que dá um gerador de números aleatórios
        gen_range -> método para definir o intervalo
        1..=100 -> número de 1 a 100
    */

    // para teste:
    // println!("The secret number is: {secret_number}");



    loop { //cria um looping infinito

        println!("Por favor, insira um número qualquer.");

        let mut guess= String::new();
        /*
            let: cria uma variável
            mut: faz a variável ser mutável
            = : associa
            String: tipo
            new(): função
            -> String::new() define uma função associada a um tipo string
        */
    
        io::stdin() //chama a função stdin 
            .read_line(&mut guess) //método da função para ler o input na linha
            .expect("Falhou em ler a linha!");
        /*
            // alter: io::stdin().readline(&mut guess).expect("Failed to read line");
            o método .read_line() retorna um valor Result, o result pode ser to tipo Ok ou Err.
            - Err: quer dizer que a operação falhou
            já o método .expect(mensagem) crasha o programa caso read_line retorne Err, e imprime a mensagem.
        */
    
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, //arm 1
            Err(_) => continue, //arm 2
        };
        /*
            trim: retirar linhas em branco e \n 
            parse: converte string em u32 (retorona num) e retorna Result (se Result é Err, quer dizer que não é convertível)
            match: compara o Result com os braços arm 1 e 2:
                - se o Result for Ok, retorna num
                - se o Result for Err, continua pro próximo looping -> "_" representa qualquer informação retornada pelo erro
            como guess é inicialmente uma String, é necessário tranformar em um número para a comparação
            isso sobre põe o valor e tipo anterior da variável guess (Shadowing)
            u32 é o tipo unsigned de 32 bits
        */

        println!("Você chutou: {guess}"); // {} é onde o valor da variável é inserido

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muito pequeno!"), // cada linha é um arm do match
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Acertou!");
                break; //quebra o looping caso acerte
            }
        }

        /*
            guess.cmp(&secret_number) -> vai comparar guess com secrect_number e retornar um tipo Ordering (::Less ou ::Greater ou ::Equal)
            match -> vai comparar o Ordering retornado pelo método cmp, e no arm que se encaixar vai executar o println!
        */
    }

}
