use std::io;

// Definindo os estados do cadeado
#[derive(PartialEq)]
enum StatePadlock {
    Locked,
    Unlocked,
    Blocked,
}

// Definindo as entradas do cadeado
enum InputPadlock {
    EnterPassword(String),
    TurnTheCrank,
}

// Definindo a estrutura da máquina de cadeado
struct MachinePadlock {
    current_state: StatePadlock,
    correct_password: String,
}

impl MachinePadlock {
    fn new(password: &str) -> Self {
        MachinePadlock {
            current_state: StatePadlock::Locked,
            correct_password: password.to_string(),
        }
    }

    fn transicao(&mut self, input: InputPadlock) {
        match input {
            InputPadlock::EnterPassword(password) => {
                if self.current_state == StatePadlock::Locked {
                    if password == self.correct_password {
                        println!("Cadeado destrancado.");
                        self.current_state = StatePadlock::Unlocked;
                    } else {
                        println!("Senha incorreta.");
                    }
                } else {
                    println!("O cadeado já está destrancado ou bloqueado.");
                }
            }
            InputPadlock::TurnTheCrank => match self.current_state {
                StatePadlock::Unlocked => {
                    println!("Cadeado bloqueado.");
                    self.current_state = StatePadlock::Blocked;
                }
                StatePadlock::Blocked => {
                    println!("Cadeado destrancado.");
                    self.current_state = StatePadlock::Unlocked;
                }
                _ => println!("Você não pode girar a manivela neste estado."),
            },
        }
    }
}

fn main() {
    let correct_password = "1234".to_string(); // Defina a senha correta aqui
    let mut padlock = MachinePadlock::new(&correct_password);

    loop {
        println!("Digite uma opção:");
        println!("1 - Girar a manivela");
        println!("2 - Digitar a senha");
        println!("3 - Sair");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a entrada");

        match input.trim() {
            "1" => padlock.transicao(InputPadlock::TurnTheCrank),
            "2" => {
                println!("Digite a senha:");
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Falha ao ler a senha");
                padlock.transicao(InputPadlock::EnterPassword(password.trim().to_string()));
            }
            "3" => break,
            _ => println!("Opção inválida"),
        }
    }
}
