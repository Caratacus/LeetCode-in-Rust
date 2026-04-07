// Tests for Problem 1830: Minimum Number of Operations to Make String Sorted
// Java reference: src/test/java/g1801_1900/s1830_minimum_number_of_operations_to_make_string_sorted/SolutionTest.java

use leetcode_in_rust::s1830::minimum_number_of_operations_to_make_string_sorted::Solution;

#[test]
fn test_make_string_sorted() {
    assert_eq!(Solution::make_string_sorted("cba".to_string()), 5);
}

#[test]
fn test_make_string_sorted2() {
    assert_eq!(Solution::make_string_sorted("aabaa".to_string()), 2);
}
