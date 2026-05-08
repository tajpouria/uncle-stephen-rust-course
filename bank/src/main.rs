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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(acc: Account) {
    println!("{:#?}", acc);
}

fn main() {
    let account = Account::new(1, String::from("Stephan"));
    let mut _bank = Bank::new();
    print_account(account);
    print_account(account);
}
