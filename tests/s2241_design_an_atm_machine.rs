// Tests for Problem 2241: Design an ATM Machine
// Java reference: src/test/java/g2201_2300/s2241_design_an_atm_machine/ATMTest.java

use leetcode_in_rust::s2241::design_an_atm_machine::ATM;

#[test]
fn test_atm() {
    let mut atm = ATM::new();
    atm.deposit(vec![0, 0, 1, 2, 1]);
    // Deposits 1 $100 banknote, 2 $200 banknotes, and 1 $500 banknote.
    assert_eq!(atm.withdraw(600), vec![0, 0, 1, 0, 1]);
    // Returns [0,0,1,0,1]. The machine uses 1 $100 banknote and 1 $500 banknote.
    atm.deposit(vec![0, 1, 0, 1, 1]);
    // Deposits 1 $50, $200, and $500 banknote.
    assert_eq!(atm.withdraw(600), vec![-1]);
    // Returns [-1]. The machine will try to use a $500 banknote
    // and then be unable to complete the remaining $100.
    assert_eq!(atm.withdraw(550), vec![0, 1, 0, 0, 1]);
    // Returns [0,1,0,0,1]. The machine uses 1 $50 banknote and 1 $500 banknote.
}
