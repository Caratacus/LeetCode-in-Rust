// Tests for Problem 2035: Partition Array Into Two Arrays to Minimize Sum Difference
// Java reference: src/test/java/g2001_2100/s2035_partition_array_into_two_arrays_to_minimize_sum_difference/SolutionTest.java

use leetcode_in_rust::s2035::partition_array_into_two_arrays_to_minimize_sum_difference::Solution;

#[test]
fn test_minimum_difference() {
    assert_eq!(Solution::minimum_difference(vec![3, 9, 7, 3]), 2);
}

#[test]
fn test_minimum_difference2() {
    assert_eq!(Solution::minimum_difference(vec![-36, 36]), 72);
}

#[test]
fn test_minimum_difference3() {
    assert_eq!(Solution::minimum_difference(vec![2, -1, 0, 4, -2, -9]), 0);
}
