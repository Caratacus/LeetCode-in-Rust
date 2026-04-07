// Tests for Problem 2855: Minimum Right Shifts to Sort the Array
// Java reference: src/test/java/g2801_2900/s2855_minimum_right_shifts_to_sort_the_array/SolutionTest.java

use leetcode_in_rust::s2855::minimum_right_shifts_to_sort_the_array::Solution;

#[test]
fn test_minimum_right_shifts() {
    assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
}

#[test]
fn test_minimum_right_shifts2() {
    assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
}

#[test]
fn test_minimum_right_shifts3() {
    assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
}
