fn main() {
    println!("************ Welcome to ADCB Bank *********");

    let mut account: BankAccount = BankAccount {
        owner: "Immanuel".to_string(),
        balance: 34567765.2772,
    };
    //immutable borrow;
    account.check_balance();
    //mutable borrow
    account.withDraw(3.456);
}
struct BankAccount {
    owner: String,
    balance: f64,
}
//self is nothing but this
impl BankAccount {
    fn withDraw(&mut self, amount: f64) {
        self.balance -= amount;
        println!(
            "Amount withdraed from {} account is {} and reaming balance is {}",
            self.owner, amount, self.balance
        );
    }

    fn check_balance(&self) {
        println!("Total balance is {} ", self.balance);
    }
}
