#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount{
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0{
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64){
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(),150.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert_eq!(account.balance(), 50.0)
    }

    // Add more tests here

    #[test]
    fn test_balance(){
        //test to check balance
        let mut account = BankAccount::new(150.0);
        assert_eq!(account.balance(), 150.0)
    }

    #[test]
    fn test_negative_deposit(){
        //test to check if negative deposit is allowed
        let mut account = BankAccount::new(900.0);
        account.deposit(-50.0);
        assert_eq!(account.balance(), 900.0);
    }

    #[test]
    fn test_negative_withdraw(){
        //test to check if negative withdraw is allowed
        let mut account = BankAccount:: new(178.0);
        account.withdraw(-90.0);
        assert_eq!(account.balance(), 178.0);
    }

    #[test]
    fn test_over_balance_withdraw(){
        //test to check if you can withdraw more than current balance
        let mut account = BankAccount::new(200.0);
        account.withdraw(201.0);
        assert_eq!(account.balance(), 200.0);
    }

    #[test]
    fn test_applied_interest(){
        //test to check interest
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.05);
        assert!((account.balance() - 105.0).abs() < 1e-10);
    }

}