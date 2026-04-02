// Tests for Problem 0078: Subsets
// Java reference: src/test/java/g0001_0100/s0078_subsets/SolutionTest.java

use leetcode_in_rust::s0078::subsets::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_subsets() {
    let result = Solution::subsets(vec![1, 2, 3]);
    let expected = vec![
        vec![], vec![1], vec![1, 2], vec![1, 2, 3],
        vec![1, 3], vec![2], vec![2, 3], vec![3],
    ];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_subsets2() {
    let result = Solution::subsets(vec![0]);
    let expected = vec![vec![], vec![0]];
    assert!(compare_2d_unsorted(&result, &expected));
}
