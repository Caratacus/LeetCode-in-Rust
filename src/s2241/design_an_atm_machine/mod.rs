// Problem 2241: design an atm machine

pub struct ATM {}

impl ATM {
    pub fn new() -> Self {
        todo!()
    }

    pub fn deposit(&mut self, banknotes_count: Vec<i32>) -> () {
        todo!()
    }

    pub fn withdraw(&mut self, amount: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void atm()
    //   ATM atm = new ATM();
    //   atm.deposit(new int[] {0, 0, 1, 2, 1});
    //   // Deposits 1 $100 banknote, 2 $200 banknotes,
    //   // and 1 $500 banknote.
    //   assertThat(atm.withdraw(600), equalTo(new int[] {0, 0, 1, 0, 1}));
    //   ... (15 more lines)
    #[test]
    fn test_atm() {
        // TODO: 翻译 Java 测试
    }
}
