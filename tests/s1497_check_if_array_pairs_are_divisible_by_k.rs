// Tests for Problem 1497: Check If Array Pairs Are Divisible by k
// Java reference: src/test/java/g1401_1500/s1497_check_if_array_pairs_are_divisible_by_k/SolutionTest.java

use leetcode_in_rust::s1497::check_if_array_pairs_are_divisible_by_k::Solution;

#[test]
fn test_can_arrange() {
    assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9], 5), true);
}

#[test]
fn test_can_arrange2() {
    assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7), true);
}

#[test]
fn test_can_arrange3() {
    assert_eq!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10), false);
}
