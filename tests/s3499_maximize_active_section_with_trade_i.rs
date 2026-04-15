// Tests for Problem 3499: Maximize Active Section with Trade I
// Java reference: src/test/java/g3401_3500/s3499_maximize_active_section_with_trade_i/SolutionTest.java

use leetcode_in_rust::s3499::maximize_active_section_with_trade_i::Solution;

#[test]
fn test_max_active_sections_after_trade() {
    assert_eq!(Solution::max_active_sections_after_trade("01".to_string()), 1);
}

#[test]
fn test_max_active_sections_after_trade2() {
    assert_eq!(Solution::max_active_sections_after_trade("0100".to_string()), 4);
}

#[test]
fn test_max_active_sections_after_trade3() {
    assert_eq!(Solution::max_active_sections_after_trade("1000100".to_string()), 7);
}

#[test]
fn test_max_active_sections_after_trade4() {
    assert_eq!(Solution::max_active_sections_after_trade("01010".to_string()), 4);
}
