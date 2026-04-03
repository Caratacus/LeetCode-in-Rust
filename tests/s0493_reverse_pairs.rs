// Tests for Problem 0493: Reverse Pairs
// Java reference: src/test/java/g0401_0500/s0493_reverse_pairs/SolutionTest.java

use leetcode_in_rust::s0493::reverse_pairs::Solution;

#[test]
fn test_reverse_pairs() {
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
}

#[test]
fn test_reverse_pairs2() {
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
}
