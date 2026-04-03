// Tests for Problem 1716: Calculate Money in Leetcode Bank
// Java reference: src/test/java/g1701_1800/s1716_calculate_money_in_leetcode_bank/SolutionTest.java

use leetcode_in_rust::s1716::calculate_money_in_leetcode_bank::Solution;

#[test]
fn test_total_money() {
    assert_eq!(Solution::total_money(4), 10);
}

#[test]
fn test_total_money2() {
    assert_eq!(Solution::total_money(10), 37);
}

#[test]
fn test_total_money3() {
    assert_eq!(Solution::total_money(20), 96);
}
