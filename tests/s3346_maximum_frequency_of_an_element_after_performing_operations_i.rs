// Tests for Problem 3346: Maximum Frequency of an Element After Performing Operations I
// Java reference: src/test/java/g3301_3400/s3346_maximum_frequency_of_an_element_after_performing_operations_i/SolutionTest.java

use leetcode_in_rust::s3346::maximum_frequency_of_an_element_after_performing_operations_i::Solution;

#[test]
fn test_max_frequency() {
    assert_eq!(Solution::max_frequency(vec![1, 4, 5], 1, 2), 2);
}

#[test]
fn test_max_frequency2() {
    assert_eq!(Solution::max_frequency(vec![5, 11, 20, 20], 5, 1), 2);
}
