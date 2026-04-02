// Tests for Problem 0775: Global and Local Inversions
// Java reference: src/test/java/g0701_0800/s0775_global_and_local_inversions/SolutionTest.java

use leetcode_in_rust::s0775::global_and_local_inversions::Solution;

#[test]
fn test_is_ideal_permutation() {
    assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
}

#[test]
fn test_is_ideal_permutation2() {
    assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
}
