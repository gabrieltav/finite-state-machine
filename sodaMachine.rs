use std::io;

// Definindo os tipos de refrigerantes disponíveis
enum Soda {
    CocaCola,
    Pepsi,
    Sprite,
}

// Definindo a estrutura da máquina de refrigerante
struct SodaMachine {
    stock: Vec<(Soda, u32)>, // (Tipo de refrigerante, quantidade disponível)
    price: u32,              // Preço fixo de cada refrigerante em centavos
    customer_balance: u32,   // Saldo do cliente em centavos
}

impl SodaMachine {
    fn new() -> Self {
        SodaMachine {
            stock: vec![(Soda::CocaCola, 10), (Soda::Pepsi, 10), (Soda::Sprite, 10)],
            price: 100, // Preço de cada refrigerante em centavos (exemplo: R$1,00)
            customer_balance: 0,
        }
    }

    fn select_soda(&mut self, choice: usize) {
        if choice < 1 || choice > self.stock.len() {
            println!("Seleção inválida.");
            return;
        }

        let (soda, available_quantity) = &mut self.stock[choice - 1];
        if *available_quantity == 0 {
            println!("Desculpe, o {} está esgotado.", name_soda(soda));
            return;
        }

        if self.customer_balance < self.price {
            println!(
                "Por favor, insira {} centavos para comprar um {}.",
                self.price,
                name_soda(soda)
            );
            return;
        }

        *available_quantity -= 1;
        self.customer_balance -= self.price;
        println!(
            "Você comprou um {}! Troco: {} centavos.",
            name_soda(soda),
            self.customer_balance
        );
    }

    fn insert_money(&mut self, value: u32) {
        self.customer_balance += value;
        println!("Saldo atual: {} centavos.", self.customer_balance);
    }
}

fn name_soda(soda: &Soda) -> &str {
    match soda {
        Soda::CocaCola => "Coca-Cola",
        Soda::Pepsi => "Pepsi",
        Soda::Sprite => "Sprite",
    }
}

fn main() {
    let mut machine = SodaMachine::new();

    loop {
        println!("Escolha um refrigerante:");
        println!("1 - Coca-Cola");
        println!("2 - Pepsi");
        println!("3 - Sprite");
        println!("4 - Inserir dinheiro");
        println!("5 - Sair");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");

        let choice: usize = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Opção inválida. Tente novamente.");
                continue;
            }
        };

        if choice == 5 {
            break;
        }

        if choice == 4 {
            println!("Digite o valor em centavos:");
            let mut value = String::new();
            io::stdin()
                .read_line(&mut value)
                .expect("Falha ao ler o valor");

            let value: u32 = match value.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("Valor inválido. Tente novamente.");
                    continue;
                }
            };

            machine.insert_money(value);
        } else {
            machine.select_soda(choice);
        }
    }
}
