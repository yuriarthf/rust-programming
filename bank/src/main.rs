fn main() {
    let mut account1 = BankAccount{
        account_number: 1,
        holder_name: String::from("holder1"),
        balance: 0.0,
    };

    let mut account2 = BankAccount{
        account_number: 2,
        holder_name: String::from("holder2"),
        balance: 10.0,
    };

    account1.deposit(10.0);
    account2.withdraw(10.0).unwrap();

    println!("Account1 balance: {}", account1.balance());
    println!("Account2 balance: {}", account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), &str>;
    fn balance(&self) -> f64;
}
#[allow(dead_code)]
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), &str> {
        if self.balance < amount {
            return Err("Not enough balance");
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
