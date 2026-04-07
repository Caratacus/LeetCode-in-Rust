// Tests for Problem 2830: Maximize the Profit as the Salesman
// Java reference: src/test/java/g2801_2900/s2830_maximize_the_profit_as_the_salesman/SolutionTest.java

use leetcode_in_rust::s2830::maximize_the_profit_as_the_salesman::Solution;

#[test]
fn test_maximize_the_profit() {
    assert_eq!(
        Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]]),
        3
    );
}

#[test]
fn test_maximize_the_profit2() {
    assert_eq!(
        Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]]),
        10
    );
}
