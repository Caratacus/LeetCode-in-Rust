// Tests for Problem 0047: Permutations II
// Java reference: src/test/java/g0001_0100/s0047_permutations_ii/SolutionTest.java

use leetcode_in_rust::s0047::permutations_ii::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_permute_unique() {
    let result = Solution::permute_unique(vec![1, 1, 2]);
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_permute_unique2() {
    let result = Solution::permute_unique(vec![1, 2, 3]);
    let expected = vec![
        vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3],
        vec![2, 3, 1], vec![3, 2, 1], vec![3, 1, 2],
    ];
    assert!(compare_2d_unsorted(&result, &expected));
}
