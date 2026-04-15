// Tests for Problem 3470: Permutations IV
// Java reference: src/test/java/g3401_3500/s3470_permutations_iv/SolutionTest.java

use leetcode_in_rust::s3470::permutations_iv::Solution;

#[test]
fn test_permute() {
    assert_eq!(Solution::permute(4, 6), vec![3, 4, 1, 2]);
}

#[test]
fn test_permute2() {
    assert_eq!(Solution::permute(3, 2), vec![3, 2, 1]);
}

#[test]
fn test_permute3() {
    assert_eq!(Solution::permute(2, 3), vec![]);
}

#[test]
fn test_permute4() {
    assert_eq!(
        Solution::permute(43, 142570305460935),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 43, 40, 27, 36, 25, 34, 31, 32, 29, 28, 33, 24, 23, 26, 41, 42, 35, 38, 37, 30, 39]
    );
}
