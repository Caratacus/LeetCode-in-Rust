// Tests for Problem 1648: Sell Diminishing-Valued Colored Balls
// Java reference: src/test/java/g1601_1700/s1648_sell_diminishing_valued_colored_balls/SolutionTest.java

use leetcode_in_rust::s1648::sell_diminishing_valued_colored_balls::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![2, 5], 4), 14);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![3, 5], 6), 19);
}
