// Tests for Problem 2972: Count the Number of Incremovable Subarrays II
// Java reference: src/test/java/g2901_3000/s2972_count_the_number_of_incremovable_subarrays_ii/SolutionTest.java

use leetcode_in_rust::s2972::count_the_number_of_incremovable_subarrays_ii::Solution;

#[test]
fn test_incremovable_subarray_count() {
    assert_eq!(Solution::incremovable_subarray_count(vec![1, 2, 3, 4]), 10);
}

#[test]
fn test_incremovable_subarray_count2() {
    assert_eq!(Solution::incremovable_subarray_count(vec![6, 5, 7, 8]), 7);
}

#[test]
fn test_incremovable_subarray_count3() {
    assert_eq!(Solution::incremovable_subarray_count(vec![8, 7, 6, 6]), 3);
}
