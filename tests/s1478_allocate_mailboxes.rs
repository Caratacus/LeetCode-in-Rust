// Tests for Problem 1478: Allocate Mailboxes
// Java reference: src/test/java/g1401_1500/s1478_allocate_mailboxes/SolutionTest.java

use leetcode_in_rust::s1478::allocate_mailboxes::Solution;

#[test]
fn test_min_distance() {
    assert_eq!(Solution::min_distance(vec![1, 4, 8, 10, 20], 3), 5);
}

#[test]
fn test_min_distance2() {
    assert_eq!(Solution::min_distance(vec![2, 3, 5, 12, 18], 2), 9);
}
