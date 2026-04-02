// Tests for Problem 0315: Count of Smaller Numbers After Self
// Java reference: src/test/java/g0301_0400/s0315_count_of_smaller_numbers_after_self/SolutionTest.java

use leetcode_in_rust::s0315::count_of_smaller_numbers_after_self::Solution;

#[test]
fn test_count_smaller() {
    assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
}

#[test]
fn test_count_smaller2() {
    assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
}

#[test]
fn test_count_smaller3() {
    assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
}
