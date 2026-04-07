// Tests for Problem 2097: Valid Arrangement of Pairs
// Java reference: src/test/java/g2001_2100/s2097_valid_arrangement_of_pairs/SolutionTest.java

use leetcode_in_rust::s2097::valid_arrangement_of_pairs::Solution;

#[test]
fn test_valid_arrangement() {
    assert_eq!(
        Solution::valid_arrangement(vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]]),
        vec![vec![11, 9], vec![9, 4], vec![4, 5], vec![5, 1]]
    );
}

#[test]
fn test_valid_arrangement2() {
    assert_eq!(
        Solution::valid_arrangement(vec![vec![1, 3], vec![3, 2], vec![2, 1]]),
        vec![vec![3, 2], vec![2, 1], vec![1, 3]]
    );
}

#[test]
fn test_valid_arrangement3() {
    assert_eq!(
        Solution::valid_arrangement(vec![vec![1, 2], vec![1, 3], vec![2, 1]]),
        vec![vec![1, 2], vec![2, 1], vec![1, 3]]
    );
}
