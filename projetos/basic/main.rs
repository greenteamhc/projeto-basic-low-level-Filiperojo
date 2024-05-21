use std::io::{self, Write};

fn main() {
    loop {
        // menu (tem que arrumar a potência)
        println!("Escolha a operação:");
        println!("1. Adição");
        println!("2. Subtração");
        println!("3. Multiplicação");
        println!("4. Divisão");
        println!("5. Potência");
        println!("6. Radiciação");
        println!("7. Porcentagem");
        println!("8. Sair");

    
        print!("Digite sua escolha: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Erro ao ler a linha");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Escolha inválida, tente novamente.");
                continue;
            }
        };

        if choice == 8 {
            println!(".");
            break;
        }

        // num1
        print!("Digite o primeiro número: ");
        io::stdout().flush().unwrap();
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Erro ao ler a linha");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Número inválido, tente novamente.");
                continue;
            }
        };

        //num2
        print!("Digite o segundo número: ");
        io::stdout().flush().unwrap();
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Erro ao ler a linha");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Número inválido.");
                continue;
            }
        };
        // escolha da operação
        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => num1 / num2,
            5 => num1.powf(num2),
            6 => num1.powf(1.0 / num2),
            7 => (num1 * 0.01) * num2,
            _ => {
                println!("Escolha inválida..");
                continue;
            }
        };

        println!("Resultado: {}", result);
    }
}
