// Tests for Problem 2578: Split With Minimum Sum
// Java reference: src/test/java/g2501_2600/s2578_split_with_minimum_sum/SolutionTest.java

use leetcode_in_rust::s2578::split_with_minimum_sum::Solution;

#[test]
fn test_split_num() {
    assert_eq!(Solution::split_num(4325), 59);
}

#[test]
fn test_split_num2() {
    assert_eq!(Solution::split_num(687), 75);
}
