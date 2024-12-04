use blockchain_simulation::Blockchain;
use std::io;

pub fn main() {
    let mut blockchain = Blockchain::new();

    let mut op = 0;
    while op != 4 {
        println!("1. Create transaction");
        println!("2. Check integrity");
        println!("3. Corrupt blockchain");
        println!("4. Exit");
        println!("Choose an option: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        op = input.trim().parse().unwrap();

        match op {
            1 => {
                println!("From: ");
                let mut from = String::new();
                io::stdin().read_line(&mut from).unwrap();
                from = from.trim().to_string();

                println!("To: ");
                let mut to = String::new();
                io::stdin().read_line(&mut to).unwrap();
                to = to.trim().to_string();

                println!("Amount: ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                let amount: f64 = amount.trim().parse().unwrap();

                blockchain.create_transaction(&from.to_string(), &to.to_string(), amount);
            }
            2 => {
                if blockchain.integrity() {
                    println!("Blockchain is valid");
                } else {
                    println!("Blockchain is corrupted");
                }
            }
            3 => {
                blockchain.corrupt();
                println!("Blockchain corrupted");
            }
            4 => {
                println!("Exiting...");
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}
