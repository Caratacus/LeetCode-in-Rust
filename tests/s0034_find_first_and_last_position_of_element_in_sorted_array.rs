// Tests for Problem 0034: Find First and Last Position of Element in Sorted Array
// Java reference: src/test/java/g0001_0100/s0034_find_first_and_last_position_of_element_in_sorted_array/SolutionTest.java

use leetcode_in_rust::s0034::find_first_and_last_position_of_element_in_sorted_array::Solution;

#[test]
fn test_search_range() {
    assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
}

#[test]
fn test_search_range2() {
    assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
}

#[test]
fn test_search_range3() {
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}
