// Tests for Problem 1655: Distribute Repeating Integers
// Java reference: src/test/java/g1601_1700/s1655_distribute_repeating_integers/SolutionTest.java

use leetcode_in_rust::s1655::distribute_repeating_integers::Solution;

#[test]
fn test_can_distribute() {
    assert_eq!(Solution::can_distribute(vec![1, 2, 3, 4], vec![2]), false);
}

#[test]
fn test_can_distribute2() {
    assert_eq!(Solution::can_distribute(vec![1, 2, 3, 3], vec![2]), true);
}

#[test]
fn test_can_distribute3() {
    assert_eq!(Solution::can_distribute(vec![1, 1, 2, 2], vec![2, 2]), true);
}
