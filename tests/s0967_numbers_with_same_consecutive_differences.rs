// Tests for Problem 0967: Numbers With Same Consecutive Differences
// Java reference: src/test/java/g0901_1000/s0967_numbers_with_same_consecutive_differences/SolutionTest.java

use leetcode_in_rust::s0967::numbers_with_same_consecutive_differences::Solution;
use std::collections::HashSet;

fn sorted_unique(vec: Vec<i32>) -> Vec<i32> {
    let set: HashSet<i32> = vec.into_iter().collect();
    let mut result: Vec<i32> = set.into_iter().collect();
    result.sort();
    result
}

#[test]
fn test_nums_same_consec_diff() {
    let result = Solution::nums_same_consec_diff(3, 7);
    let mut expected = vec![181, 292, 707, 818, 929];
    expected.sort();
    assert_eq!(sorted_unique(result), expected);
}

#[test]
fn test_nums_same_consec_diff2() {
    let result = Solution::nums_same_consec_diff(2, 1);
    let mut expected = vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98];
    expected.sort();
    assert_eq!(sorted_unique(result), expected);
}
