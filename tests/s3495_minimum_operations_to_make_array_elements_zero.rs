// Tests for Problem 3495: Minimum Operations to Make Array Elements Zero
// Java reference: src/test/java/g3401_3500/s3495_minimum_operations_to_make_array_elements_zero/SolutionTest.java

use leetcode_in_rust::s3495::minimum_operations_to_make_array_elements_zero::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![vec![1, 2], vec![2, 4]]), 3i64);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![vec![2, 6]]), 4i64);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![vec![5, 8]]), 4i64);
}

#[test]
fn test_min_operations4() {
    assert_eq!(Solution::min_operations(vec![vec![1, 21]]), 23i64);
}
