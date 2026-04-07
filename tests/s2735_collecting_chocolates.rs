// Tests for Problem 2735: Collecting Chocolates
// Java reference: src/test/java/g2701_2800/s2735_collecting_chocolates/SolutionTest.java

use leetcode_in_rust::s2735::collecting_chocolates::Solution;

#[test]
fn test_min_cost() {
    assert_eq!(Solution::min_cost(vec![20, 1, 15], 5), 13);
}

#[test]
fn test_min_cost2() {
    assert_eq!(Solution::min_cost(vec![1, 2, 3], 4), 6);
}
