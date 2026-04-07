// Tests for Problem 2811: Check if it is Possible to Split Array
// Java reference: src/test/java/g2801_2900/s2811_check_if_it_is_possible_to_split_array/SolutionTest.java

use leetcode_in_rust::s2811::check_if_it_is_possible_to_split_array::Solution;

#[test]
fn test_can_split_array() {
    assert_eq!(Solution::can_split_array(vec![2, 2, 1], 4), true);
}

#[test]
fn test_can_split_array2() {
    assert_eq!(Solution::can_split_array(vec![2, 1, 3], 5), false);
}

#[test]
fn test_can_split_array3() {
    assert_eq!(Solution::can_split_array(vec![2, 3, 3, 2, 3], 6), true);
}

#[test]
fn test_can_split_array4() {
    assert_eq!(Solution::can_split_array(vec![1], 1), true);
}
