// Tests for Problem 3289: The Two Sneaky Numbers of Digitville
// Java reference: src/test/java/g3201_3300/s3289_the_two_sneaky_numbers_of_digitville/SolutionTest.java

use leetcode_in_rust::s3289::the_two_sneaky_numbers_of_digitville::Solution;

#[test]
fn test_get_sneaky_numbers() {
    assert_eq!(Solution::get_sneaky_numbers(vec![0, 1, 1, 0]), vec![0, 1]);
}

#[test]
fn test_get_sneaky_numbers2() {
    assert_eq!(Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]), vec![2, 3]);
}

#[test]
fn test_get_sneaky_numbers3() {
    assert_eq!(Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]), vec![4, 5]);
}
