// Tests for Problem 3255: Find the Power of K-Size Subarrays II
// Java reference: src/test/java/g3201_3300/s3255_find_the_power_of_k_size_subarrays_ii/SolutionTest.java

use leetcode_in_rust::s3255::find_the_power_of_k_size_subarrays_ii::Solution;

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

#[test]
fn test_results_array4() {
    assert_eq!(Solution::results_array(vec![1], 1), vec![1]);
}
