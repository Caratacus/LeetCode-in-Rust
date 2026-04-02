// Tests for Problem 0870: Advantage Shuffle
// Java reference: src/test/java/g0801_0900/s0870_advantage_shuffle/SolutionTest.java

use leetcode_in_rust::s0870::advantage_shuffle::Solution;

#[test]
fn test_advantage_count() {
    assert_eq!(
        Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
}

#[test]
fn test_advantage_count2() {
    assert_eq!(
        Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}
