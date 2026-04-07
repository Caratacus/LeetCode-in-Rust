// Tests for Problem 2483: Minimum Penalty for a Shop
// Java reference: src/test/java/g2401_2500/s2483_minimum_penalty_for_a_shop/SolutionTest.java

use leetcode_in_rust::s2483::minimum_penalty_for_a_shop::Solution;

#[test]
fn test_best_closing_time() {
    assert_eq!(Solution::best_closing_time("NNNNN".to_string()), 0);
}

#[test]
fn test_best_closing_time2() {
    assert_eq!(Solution::best_closing_time("YYYY".to_string()), 4);
}
