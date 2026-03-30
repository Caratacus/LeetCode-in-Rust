// Problem 2043: simple bank system

pub struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    pub fn new(balance: Vec<i64>) -> Self {
        todo!()
    }

    pub fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        todo!()
    }

    pub fn deposit(&mut self, account: i32, money: i64) -> bool {
        todo!()
    }

    pub fn withdraw(&mut self, account: i32, money: i64) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void bankTest()
    //   Bank bank = new Bank(new long[] {10, 100, 20, 50, 30});
    //   assertThat(bank.withdraw(3, 10), equalTo(true));
    //   assertThat(bank.transfer(5, 1, 20), equalTo(true));
    //   assertThat(bank.deposit(5, 20), equalTo(true));
    //   assertThat(bank.transfer(3, 4, 15), equalTo(false));
    //   ... (1 more lines)
    #[test]
    fn test_bank_test() {
        // TODO: 翻译 Java 测试
    }
}
