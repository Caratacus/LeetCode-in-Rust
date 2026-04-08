// Tests for Problem 3012: Minimize Length of Array Using Operations
// Java reference: src/test/java/g3001_3100/s3012_minimize_length_of_array_using_operations/SolutionTest.java

use leetcode_in_rust::s3012::minimize_length_of_array_using_operations::Solution;

#[test]
fn test_minimum_array_length() {
    assert_eq!(Solution::minimum_array_length(vec![1, 4, 3, 1]), 1);
}

#[test]
fn test_minimum_array_length2() {
    assert_eq!(Solution::minimum_array_length(vec![5, 5, 5, 10, 5]), 2);
}

#[test]
fn test_minimum_array_length3() {
    assert_eq!(Solution::minimum_array_length(vec![2, 3, 4]), 1);
}
