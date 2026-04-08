// Tests for Problem 3113: Find the Number of Subarrays Where Boundary Elements are Maximum
// Java reference: src/test/java/g3101_3200/s3113_find_the_number_of_subarrays_where_boundary_elements_are_maximum/SolutionTest.java

use leetcode_in_rust::s3113::find_the_number_of_subarrays_where_boundary_elements_are_maximum::Solution;

#[test]
fn test_number_of_subarrays() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
}

#[test]
fn test_number_of_subarrays2() {
    assert_eq!(Solution::number_of_subarrays(vec![3, 3, 3]), 6);
}

#[test]
fn test_number_of_subarrays3() {
    assert_eq!(Solution::number_of_subarrays(vec![1]), 1);
}
