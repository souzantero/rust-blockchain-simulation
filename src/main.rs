pub type Id = i32;
pub type Timestamp = i64;
pub type AccountId = String;
pub type Hash = String;
pub type Amount = f64;

pub struct Transaction {
    pub id: Id,
    pub origin: AccountId,
    pub destination: AccountId,
    pub amount: Amount,
}

pub struct Block {
    pub id: Id,
    pub timestamp: Timestamp,
    pub transactions: Vec<Transaction>,
    pub previous_hash: Hash,
    pub hash: Hash,
}

impl Block {
    pub fn new(id: Id, previous_hash: Hash) -> Self {
        let timestamp = 0;
        let transactions: Vec<Transaction> = Vec::new();
        let hash = "".to_string();
        Block {
            id,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: Vec::new() }
    }

    pub fn create_transaction(&mut self, origin: &str, destination: &str, amount: Amount) {
        let transaction = Transaction {
            id: 1,
            origin: origin.to_string(),
            destination: destination.to_string(),
            amount,
        };

        let current_block = self.get_current_block();
        current_block.transactions.push(transaction);
    }

    fn get_current_block(&mut self) -> &mut Block {
        if let Some(current_block) = self.blocks.last_mut() {
            if current_block.transactions.len() >= 5 {
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
}
