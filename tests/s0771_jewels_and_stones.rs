// Tests for Problem 0771: Jewels and Stones
// Java reference: src/test/java/g0701_0800/s0771_jewels_and_stones/SolutionTest.java

use leetcode_in_rust::s0771::jewels_and_stones::Solution;

#[test]
fn test_num_jewels_in_stones() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
}

#[test]
fn test_num_jewels_in_stones2() {
    assert_eq!(
        Solution::num_jewels_in_stones("z".to_string(), "ZZ".to_string()),
        0
    );
}
