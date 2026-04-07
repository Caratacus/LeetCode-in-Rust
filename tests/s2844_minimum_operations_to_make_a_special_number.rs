// Tests for Problem 2844: Minimum Operations to Make a Special Number
// Java reference: src/test/java/g2801_2900/s2844_minimum_operations_to_make_a_special_number/SolutionTest.java

use leetcode_in_rust::s2844::minimum_operations_to_make_a_special_number::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations("2245047".to_string()), 2);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations("2908305".to_string()), 3);
}

#[test]
fn test_minimum_operations3() {
    assert_eq!(Solution::minimum_operations("10".to_string()), 1);
}
