// Tests for Problem 2090: K Radius Subarray Averages
// Java reference: src/test/java/g2001_2100/s2090_k_radius_subarray_averages/SolutionTest.java

use leetcode_in_rust::s2090::k_radius_subarray_averages::Solution;

#[test]
fn test_get_averages() {
    assert_eq!(
        Solution::get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
        vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
    );
}

#[test]
fn test_get_averages2() {
    assert_eq!(Solution::get_averages(vec![100000], 0), vec![100000]);
}

#[test]
fn test_get_averages3() {
    assert_eq!(Solution::get_averages(vec![8], 100000), vec![-1]);
}
