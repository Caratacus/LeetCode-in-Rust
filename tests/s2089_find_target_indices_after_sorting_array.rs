// Tests for Problem 2089: Find Target Indices After Sorting Array
// Java reference: src/test/java/g2001_2100/s2089_find_target_indices_after_sorting_array/SolutionTest.java

use leetcode_in_rust::s2089::find_target_indices_after_sorting_array::Solution;

#[test]
fn test_target_indices() {
    assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
}

#[test]
fn test_target_indices2() {
    assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 3), vec![3]);
}

#[test]
fn test_target_indices3() {
    assert_eq!(Solution::target_indices(vec![1, 2, 5, 2, 3], 5), vec![4]);
}
