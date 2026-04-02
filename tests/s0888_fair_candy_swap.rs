// Tests for Problem 0888: Fair Candy Swap
// Java reference: src/test/java/g0801_0900/s0888_fair_candy_swap/SolutionTest.java

use leetcode_in_rust::s0888::fair_candy_swap::Solution;

#[test]
fn test_fair_candy_swap() {
    assert_eq!(Solution::fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
}

#[test]
fn test_fair_candy_swap2() {
    assert_eq!(Solution::fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
}

#[test]
fn test_fair_candy_swap3() {
    assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
}
