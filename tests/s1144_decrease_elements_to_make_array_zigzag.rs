// Tests for Problem 1144: Decrease Elements to Make Array Zigzag
// Java reference: src/test/java/g1101_1200/s1144_decrease_elements_to_make_array_zigzag/SolutionTest.java

use leetcode_in_rust::s1144::decrease_elements_to_make_array_zigzag::Solution;

#[test]
fn test_moves_to_make_zigzag() {
    assert_eq!(Solution::moves_to_make_zigzag(vec![1, 2, 3]), 2);
}

#[test]
fn test_moves_to_make_zigzag2() {
    assert_eq!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
}
