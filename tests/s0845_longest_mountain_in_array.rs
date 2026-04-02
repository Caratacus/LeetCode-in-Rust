// Tests for Problem 0845: Longest Mountain in Array
// Java reference: src/test/java/g0801_0900/s0845_longest_mountain_in_array/SolutionTest.java

use leetcode_in_rust::s0845::longest_mountain_in_array::Solution;

#[test]
fn test_longest_mountain() {
    assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
}

#[test]
fn test_longest_mountain2() {
    assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
}
