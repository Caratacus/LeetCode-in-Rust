// Tests for Problem 0228: Summary Ranges
// Java reference: src/test/java/g0201_0300/s0228_summary_ranges/SolutionTest.java

use leetcode_in_rust::s0228::summary_ranges::Solution;

#[test]
fn test_summary_ranges() {
    assert_eq!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]), vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]);
}

#[test]
fn test_summary_ranges2() {
    assert_eq!(Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]), vec!["0".to_string(), "2->4".to_string(), "6".to_string(), "8->9".to_string()]);
}
