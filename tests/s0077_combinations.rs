// Tests for Problem 0077: Combinations
// Java reference: src/test/java/g0001_0100/s0077_combinations/SolutionTest.java

use leetcode_in_rust::s0077::combinations::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_combine() {
    let result = Solution::combine(4, 2);
    let expected = vec![
        vec![2, 1], vec![3, 1], vec![4, 1],
        vec![3, 2], vec![4, 2], vec![4, 3],
    ];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_combine2() {
    let result = Solution::combine(1, 1);
    let expected = vec![vec![1]];
    assert!(compare_2d_unsorted(&result, &expected));
}
