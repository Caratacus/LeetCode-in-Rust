// Tests for Problem 1287: Element Appearing More Than 25% In Sorted Array
// Java reference: src/test/java/g1201_1300/s1287_element_appearing_more_than_25_in_sorted_array/SolutionTest.java

use leetcode_in_rust::s1287::element_appearing_more_than_25_in_sorted_array::Solution;

#[test]
fn test_find_special_integer() {
    assert_eq!(
        Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
        6
    );
}

#[test]
fn test_find_special_integer2() {
    assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
}
