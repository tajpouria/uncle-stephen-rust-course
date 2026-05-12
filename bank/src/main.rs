#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Self {
            id,
            balance: 0,
            holder,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!(
            "{}: {} has balance of {}",
            self.id, self.holder, self.balance
        )
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|acc| acc.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|acc| acc.summary()).collect()
    }
}

fn main() {
    let mut account = Account::new(1, String::from("Stephan"));
    account.deposit(500);
    account.withdraw(213);

    let mut bank = Bank::new();
    bank.add_account(account);

    println!("Total balance: {}", bank.total_balance());
    println!("Summary: {:#?}", bank.summary());
}
