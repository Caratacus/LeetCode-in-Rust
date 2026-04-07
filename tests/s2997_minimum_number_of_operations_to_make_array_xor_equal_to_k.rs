// Tests for Problem 2997: Minimum Number of Operations to Make Array XOR Equal to K
// Java reference: src/test/java/g2901_3000/s2997_minimum_number_of_operations_to_make_array_xor_equal_to_k/SolutionTest.java

use leetcode_in_rust::s2997::minimum_number_of_operations_to_make_array_xor_equal_to_k::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![2, 1, 3, 4], 1), 2);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 0, 2, 0], 0), 0);
}
