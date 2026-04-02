// Tests for Problem 0090: Subsets II
// Java reference: src/test/java/g0001_0100/s0090_subsets_ii/SolutionTest.java

use leetcode_in_rust::s0090::subsets_ii::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_subsets_with_dup() {
    let result = Solution::subsets_with_dup(vec![1, 2, 2]);
    let expected = vec![
        vec![], vec![1], vec![2], vec![1, 2], vec![2, 2], vec![1, 2, 2],
    ];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_subsets_with_dup2() {
    let result = Solution::subsets_with_dup(vec![0]);
    let expected = vec![vec![], vec![0]];
    assert!(compare_2d_unsorted(&result, &expected));
}
