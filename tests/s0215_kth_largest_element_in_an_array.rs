// Tests for Problem 0215: Kth Largest Element in an Array
// Java reference: src/test/java/g0201_0300/s0215_kth_largest_element_in_an_array/SolutionTest.java

use leetcode_in_rust::s0215::kth_largest_element_in_an_array::Solution;

#[test]
fn test_find_kth_largest() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
}

#[test]
fn test_find_kth_largest2() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
}
