// Tests for Problem 0912: Sort an Array
// Java reference: src/test/java/g0901_1000/s0912_sort_an_array/SolutionTest.java

use leetcode_in_rust::s0912::sort_an_array::Solution;

#[test]
fn test_sort_array() {
    assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
}

#[test]
fn test_sort_array2() {
    assert_eq!(Solution::sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
}
