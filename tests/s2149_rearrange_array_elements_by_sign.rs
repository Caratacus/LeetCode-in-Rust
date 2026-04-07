// Tests for Problem 2149: Rearrange Array Elements by Sign
// Java reference: src/test/java/g2101_2200/s2149_rearrange_array_elements_by_sign/SolutionTest.java

use leetcode_in_rust::s2149::rearrange_array_elements_by_sign::Solution;

#[test]
fn test_rearrange_array() {
    assert_eq!(
        Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
        vec![3, -2, 1, -5, 2, -4]
    );
}

#[test]
fn test_rearrange_array2() {
    assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
}
