// Tests for Problem 2896: Apply Operations to Make Two Strings Equal
// Java reference: src/test/java/g2801_2900/s2896_apply_operations_to_make_two_strings_equal/SolutionTest.java

use leetcode_in_rust::s2896::apply_operations_to_make_two_strings_equal::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations("1100011000".to_string(), "0101001010".to_string(), 2), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations("10110".to_string(), "00011".to_string(), 4), -1);
}
