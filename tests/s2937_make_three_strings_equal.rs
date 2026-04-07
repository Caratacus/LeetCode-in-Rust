// Tests for Problem 2937: Make Three Strings Equal
// Java reference: src/test/java/g2901_3000/s2937_make_three_strings_equal/SolutionTest.java

use leetcode_in_rust::s2937::make_three_strings_equal::Solution;

#[test]
fn test_find_minimum_operations() {
    assert_eq!(
        Solution::find_minimum_operations("abc".to_string(), "abb".to_string(), "ab".to_string()),
        2
    );
}

#[test]
fn test_find_minimum_operations2() {
    assert_eq!(
        Solution::find_minimum_operations("dac".to_string(), "bac".to_string(), "cac".to_string()),
        -1
    );
}
