// Tests for Problem 0046: Permutations
// Java reference: src/test/java/g0001_0100/s0046_permutations/SolutionTest.java

use leetcode_in_rust::s0046::permutations::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_permute() {
    let result = Solution::permute(vec![1, 2, 3]);
    let expected = vec![
        vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3],
        vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1],
    ];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_permute2() {
    let result = Solution::permute(vec![0, 1]);
    let expected = vec![vec![0, 1], vec![1, 0]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_permute3() {
    let result = Solution::permute(vec![1]);
    let expected = vec![vec![1]];
    assert!(compare_2d_unsorted(&result, &expected));
}
