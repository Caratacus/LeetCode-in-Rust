// Tests for Problem 2840: Check if Strings Can be Made Equal with Operations II
// Java reference: src/test/java/g2801_2900/s2840_check_if_strings_can_be_made_equal_with_operations_ii/SolutionTest.java

use leetcode_in_rust::s2840::check_if_strings_can_be_made_equal_with_operations_ii::Solution;

#[test]
fn test_check_strings() {
    assert_eq!(Solution::check_strings("abcdba".to_string(), "cabdab".to_string()), true);
}

#[test]
fn test_check_strings2() {
    assert_eq!(Solution::check_strings("abe".to_string(), "bea".to_string()), false);
}
