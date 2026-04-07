// Tests for Problem 2546: Apply Bitwise Operations to Make Strings Equal
// Java reference: src/test/java/g2501_2600/s2546_apply_bitwise_operations_to_make_strings_equal/SolutionTest.java
use leetcode_in_rust::s2546::apply_bitwise_operations_to_make_strings_equal::Solution;

#[test]
fn test_make_strings_equal() {
    assert_eq!(Solution::make_strings_equal("1010".to_string(), "0110".to_string()), true);
}
#[test]
fn test_make_strings_equal2() {
    assert_eq!(Solution::make_strings_equal("11".to_string(), "00".to_string()), false);
}
