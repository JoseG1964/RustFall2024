mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(1000.0);
    println!("Current Balance: ${}", account.balance());

    account.deposit(50.0);
    println!("New Balance after deposit: ${}", account.balance());

    account.withdraw(50.0);
    println!("New Balance after withdraw: ${}", account.balance());

    account.apply_interest(0.05);
    println!("Balance after Interest: ${}", account.balance());
}
