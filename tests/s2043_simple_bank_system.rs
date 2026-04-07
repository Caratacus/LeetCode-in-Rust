// Tests for Problem 2043: Simple Bank System
// Java reference: src/test/java/g2001_2100/s2043_simple_bank_system/BankTest.java

use leetcode_in_rust::s2043::simple_bank_system::Bank;

#[test]
fn test_bank_test() {
    let mut bank = Bank::new(vec![10, 100, 20, 50, 30]);
    assert_eq!(bank.withdraw(3, 10), true);
    assert_eq!(bank.transfer(5, 1, 20), true);
    assert_eq!(bank.deposit(5, 20), true);
    assert_eq!(bank.transfer(3, 4, 15), false);
    assert_eq!(bank.withdraw(10, 50), false);
}
