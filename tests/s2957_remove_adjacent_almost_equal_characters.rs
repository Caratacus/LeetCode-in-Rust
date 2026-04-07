// Tests for Problem 2957: Remove Adjacent Almost-Equal Characters
// Java reference: src/test/java/g2901_3000/s2957_remove_adjacent_almost_equal_characters/SolutionTest.java

use leetcode_in_rust::s2957::remove_adjacent_almost_equal_characters::Solution;

#[test]
fn test_remove_almost_equal_characters() {
    assert_eq!(Solution::remove_almost_equal_characters("aaaaa".to_string()), 2);
}

#[test]
fn test_remove_almost_equal_characters2() {
    assert_eq!(Solution::remove_almost_equal_characters("abddez".to_string()), 2);
}

#[test]
fn test_remove_almost_equal_characters3() {
    assert_eq!(Solution::remove_almost_equal_characters("zyxyxyz".to_string()), 3);
}
