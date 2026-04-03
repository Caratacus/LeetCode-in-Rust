// Tests for Problem 1738: Find Kth Largest XOR Coordinate Value
// Java reference: src/test/java/g1701_1800/s1738_find_kth_largest_xor_coordinate_value/SolutionTest.java

use leetcode_in_rust::s1738::find_kth_largest_xor_coordinate_value::Solution;

#[test]
fn test_kth_largest_value() {
    assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
}

#[test]
fn test_kth_largest_value2() {
    assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2), 5);
}

#[test]
fn test_kth_largest_value3() {
    assert_eq!(Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3), 4);
}
