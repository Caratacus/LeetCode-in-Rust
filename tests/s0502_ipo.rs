// Tests for Problem 0502: IPO
// Java reference: src/test/java/g0501_0600/s0502_ipo/SolutionTest.java

use leetcode_in_rust::s0502::ipo::Solution;

#[test]
fn test_find_maximized_capital() {
    assert_eq!(
        Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
        4
    );
}

#[test]
fn test_find_maximized_capital2() {
    assert_eq!(
        Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
        6
    );
}
