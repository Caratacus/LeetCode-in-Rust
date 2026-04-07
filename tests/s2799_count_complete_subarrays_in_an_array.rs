// Tests for Problem 2799: Count Complete Subarrays in an Array
// Java reference: src/test/java/g2701_2800/s2799_count_complete_subarrays_in_an_array/SolutionTest.java

use leetcode_in_rust::s2799::count_complete_subarrays_in_an_array::Solution;

#[test]
fn test_count_complete_subarrays() {
    assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
}

#[test]
fn test_count_complete_subarrays2() {
    assert_eq!(Solution::count_complete_subarrays(vec![5, 5, 5, 5]), 10);
}
