// Tests for Problem 0260: Single Number III
// Java reference: src/test/java/g0201_0300/s0260_single_number_iii/SolutionTest.java

use leetcode_in_rust::s0260::single_number_iii::Solution;

#[test]
fn test_single_number() {
    let mut result = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
    result.sort();
    assert_eq!(result, vec![3, 5]);
}

#[test]
fn test_single_number2() {
    let mut result = Solution::single_number(vec![-1, 0]);
    result.sort();
    assert_eq!(result, vec![-1, 0]);
}
