// Tests for Problem 3254: Find the Power of K-Size Subarrays I
// Java reference: src/test/java/g3201_3300/s3254_find_the_power_of_k_size_subarrays_i/SolutionTest.java

use leetcode_in_rust::s3254::find_the_power_of_k_size_subarrays_i::Solution;

#[test]
fn test_results_array() {
    assert_eq!(
        Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3),
        vec![3, 4, -1, -1, -1]
    );
}

#[test]
fn test_results_array2() {
    assert_eq!(Solution::results_array(vec![2, 2, 2, 2, 2], 4), vec![-1, -1]);
}

#[test]
fn test_results_array3() {
    assert_eq!(
        Solution::results_array(vec![3, 2, 3, 2, 3, 2], 2),
        vec![-1, 3, -1, 3, -1]
    );
}
