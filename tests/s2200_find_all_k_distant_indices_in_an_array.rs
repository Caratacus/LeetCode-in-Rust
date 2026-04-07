// Tests for Problem 2200: Find All K-Distant Indices in an Array
// Java reference: src/test/java/g2101_2200/s2200_find_all_k_distant_indices_in_an_array/SolutionTest.java

use leetcode_in_rust::s2200::find_all_k_distant_indices_in_an_array::Solution;

#[test]
fn test_find_k_distant_indices() {
    assert_eq!(
        Solution::find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn test_find_k_distant_indices2() {
    assert_eq!(
        Solution::find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
        vec![0, 1, 2, 3, 4]
    );
}
