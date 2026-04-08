// Tests for Problem 3011: Find If Array Can Be Sorted
// Java reference: src/test/java/g3001_3100/s3011_find_if_array_can_be_sorted/SolutionTest.java

use leetcode_in_rust::s3011::find_if_array_can_be_sorted::Solution;

#[test]
fn test_can_sort_array() {
    assert_eq!(Solution::can_sort_array(vec![8, 4, 2, 30, 15]), true);
}

#[test]
fn test_can_sort_array2() {
    assert_eq!(Solution::can_sort_array(vec![1, 2, 3, 4, 5]), true);
}

#[test]
fn test_can_sort_array3() {
    assert_eq!(Solution::can_sort_array(vec![3, 16, 8, 4, 2]), false);
}
