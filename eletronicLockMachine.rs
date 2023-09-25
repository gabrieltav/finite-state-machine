use std::io;

// Definindo a estrutura da fechadura eletrônica
struct EletronicLock {
    corret_password: String,
    locked: bool,
}

impl EletronicLock {
    fn new(password: &str) -> Self {
        EletronicLock {
            corret_password: password.to_string(),
            locked: true,
        }
    }

    fn unlock(&mut self, password: &str) {
        if self.corret_password == password {
            self.locked = false;
            println!("Fechadura destrancada.");
        } else {
            println!("Senha incorreta. Tente novamente.");
        }
    }

    fn tolock(&mut self) {
        self.locked = true;
        println!("Fechadura trancada.");
    }

    fn its_locked(&self) -> bool {
        self.locked
    }
}

fn main() {
    let corret_password = "1234".to_string(); // Defina a senha correta aqui
    let mut lock = EletronicLock::new(&corret_password);

    loop {
        println!("Digite uma opção:");
        println!("1 - Destrancar");
        println!("2 - Trancar");
        println!("3 - Sair");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler a entrada");

        match choice.trim() {
            "1" => {
                println!("Digite a senha:");
                let mut password = String::new();
                io::stdin()
                    .read_line(&mut password)
                    .expect("Falha ao ler a senha");
                lock.unlock(password.trim());
            }
            "2" => lock.tolock(),
            "3" => break,
            _ => println!("Opção inválida"),
        }

        if lock.its_locked() {
            println!("A fechadura está trancada.");
        } else {
            println!("A fechadura está destrancada.");
        }
    }
}
