// Tests for Problem 2772: Apply Operations to Make All Array Elements Equal to Zero
// Java reference: src/test/java/g2701_2800/s2772_apply_operations_to_make_all_array_elements_equal_to_zero/SolutionTest.java

use leetcode_in_rust::s2772::apply_operations_to_make_all_array_elements_equal_to_zero::Solution;

#[test]
fn test_check_array() {
    assert_eq!(Solution::check_array(vec![2, 2, 3, 1, 1, 0], 3), true);
}

#[test]
fn test_check_array2() {
    assert_eq!(Solution::check_array(vec![1, 3, 1, 1], 2), false);
}
