// Tests for Problem 0031: Next Permutation
// Java reference: src/test/java/g0001_0100/s0031_next_permutation/SolutionTest.java

use leetcode_in_rust::s0031::next_permutation::Solution;

#[test]
fn test_next_permutation() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 3, 2]);
}

#[test]
fn test_next_permutation2() {
    let mut nums = vec![3, 2, 1];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 2, 3]);
}

#[test]
fn test_next_permutation3() {
    let mut nums = vec![1, 1, 5];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 5, 1]);
}
