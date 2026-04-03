// Tests for Problem 1608: Special Array With X Elements Greater Than or Equal X
// Java reference: src/test/java/g1601_1700/s1608_special_array_with_x_elements_greater_than_or_equal_x/SolutionTest.java

use leetcode_in_rust::s1608::special_array_with_x_elements_greater_than_or_equal_x::Solution;

#[test]
fn test_special_array() {
    assert_eq!(Solution::special_array(vec![3, 5]), 2);
}

#[test]
fn test_special_array2() {
    assert_eq!(Solution::special_array(vec![0, 0]), -1);
}

#[test]
fn test_special_array3() {
    assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
}
