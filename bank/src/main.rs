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

    match account1.deposit(10.0) {
        Ok(_) => println!("Account1 balance: {}", account1.balance()),
        Err(error) => panic!("{}", error),
    }

    match account2.withdraw(10.0) {
        Ok(_) => println!("Account2 balance: {}", account2.balance()),
        Err(error) => panic!("{}", error),
    }

}

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}
#[allow(dead_code)]
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if self.balance + amount < self.balance {
            return Err("Balance overflow".to_string())
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance < amount {
            return Err("Not enough balance".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
