// Tests for Problem 3501: Maximize Active Section with Trade II
// Java reference: src/test/java/g3501_3600/s3501_maximize_active_section_with_trade_ii/SolutionTest.java

use leetcode_in_rust::s3501::maximize_active_section_with_trade_ii::Solution;

#[test]
fn test_max_active_sections_after_trade() {
    assert_eq!(
        Solution::max_active_sections_after_trade("01".to_string(), vec![vec![0, 1]]),
        vec![1]
    );
}

#[test]
fn test_max_active_sections_after_trade2() {
    assert_eq!(
        Solution::max_active_sections_after_trade("0100".to_string(), vec![vec![0, 3], vec![0, 2], vec![1, 3], vec![2, 3]]),
        vec![4, 3, 1, 1]
    );
}

#[test]
fn test_max_active_sections_after_trade3() {
    assert_eq!(
        Solution::max_active_sections_after_trade("1000100".to_string(), vec![vec![1, 5], vec![0, 6], vec![0, 4]]),
        vec![6, 7, 2]
    );
}

#[test]
fn test_max_active_sections_after_trade4() {
    assert_eq!(
        Solution::max_active_sections_after_trade("01010".to_string(), vec![vec![0, 3], vec![1, 4], vec![1, 3]]),
        vec![4, 4, 2]
    );
}

#[test]
fn test_max_active_sections_after_trade5() {
    assert_eq!(
        Solution::max_active_sections_after_trade("10110111".to_string(), vec![vec![3, 7], vec![4, 6], vec![0, 6]]),
        vec![6, 6, 8]
    );
}
