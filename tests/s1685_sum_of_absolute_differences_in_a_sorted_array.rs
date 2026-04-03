// Tests for Problem 1685: Sum of Absolute Differences in a Sorted Array
// Java reference: src/test/java/g1601_1700/s1685_sum_of_absolute_differences_in_a_sorted_array/SolutionTest.java

use leetcode_in_rust::s1685::sum_of_absolute_differences_in_a_sorted_array::Solution;

#[test]
fn test_get_sum_absolute_differences() {
    assert_eq!(Solution::get_sum_absolute_differences(vec![2, 3, 5]), vec![4, 3, 5]);
}

#[test]
fn test_get_sum_absolute_differences2() {
    assert_eq!(
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
        vec![24, 15, 13, 15, 21]
    );
}
