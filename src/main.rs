mod account;
mod utils;

use account::BankAccount;

fn main() {
    let mut account = BankAccount::new(0.0); // Start with zero balance

    loop {
        println!("Choose an action: \n1. Deposit \n2. Withdraw \n3. Check Balance \n4. Exit");

        let choice = utils::get_input().trim().parse::<u32>();
        match choice {
            Ok(1) => {
                println!("Enter deposit amount:");
                let amount = utils::get_input().trim().parse::<f64>().unwrap_or(0.0);
                account.deposit(amount);
            }
            Ok(2) => {
                println!("Enter withdrawal amount:");
                let amount = utils::get_input().trim().parse::<f64>().unwrap_or(0.0);
                account.withdraw(amount);
            }
            Ok(3) => account.check_balance(),
            Ok(4) => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Try again."),
        }
    }
}
