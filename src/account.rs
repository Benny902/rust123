pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Constructor to create a new account
    pub fn new(balance: f64) -> Self {
        Self { balance }
    }

    // Method to deposit money into the account
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited: ${}", amount);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    // Method to withdraw money from the account
    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!("Withdrew: ${}", amount);
        } else {
            println!("Insufficient balance or invalid amount.");
        }
    }

    // Method to display the current balance
    pub fn check_balance(&self) {
        println!("Current balance: ${}", self.balance);
    }
}
