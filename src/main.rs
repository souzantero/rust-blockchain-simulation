use std::time::SystemTime;

const MAX_TRANSACTIONS_PER_BLOCK: usize = 5;

pub type Id = u32;
pub type Timestamp = u64;
pub type AccountId = String;
pub type Hash = String;
pub type Amount = f64;

#[derive(Debug)]
pub struct Transaction {
    pub id: Id,
    pub origin: AccountId,
    pub destination: AccountId,
    pub amount: Amount,
}

#[derive(Debug)]
pub struct Block {
    pub id: Id,
    pub timestamp: Timestamp,
    pub transactions: Vec<Transaction>,
    pub previous_hash: Hash,
    pub hash: Hash,
}

impl Block {
    pub fn new(id: Id, previous_hash: Hash) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hash = format!("{:?}", (id, &previous_hash, timestamp));
        let transactions: Vec<Transaction> = Vec::new();
        Block {
            id,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: Vec::new() }
    }

    pub fn create_transaction(&mut self, origin: &str, destination: &str, amount: Amount) {
        let current_block = self.get_current_block();
        let last_transaction = current_block.transactions.last();
        let id = match last_transaction {
            Some(transaction) => transaction.id + 1,
            None => 1,
        };

        let transaction = Transaction {
            id,
            origin: origin.to_string(),
            destination: destination.to_string(),
            amount,
        };

        current_block.transactions.push(transaction);
    }

    fn get_current_block(&mut self) -> &mut Block {
        if let Some(current_block) = self.blocks.last_mut() {
            if current_block.transactions.len() >= MAX_TRANSACTIONS_PER_BLOCK {
                let new_block = Block::new(current_block.id + 1, current_block.hash.clone());
                self.blocks.push(new_block);
            }
        } else {
            let new_block = Block::new(1, "".to_string());
            self.blocks.push(new_block);
        }

        self.blocks.last_mut().unwrap()
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.create_transaction("Alice", "Bob", 100.0);
    blockchain.create_transaction("Bob", "Alice", 50.0);
    blockchain.create_transaction("Alice", "Bob", 25.0);
    blockchain.create_transaction("Alice", "Bob", 10.0);
    blockchain.create_transaction("Bob", "Alice", 5.0);
    blockchain.create_transaction("Alice", "Bob", 2.5);
    println!("{:?}", blockchain);
}
