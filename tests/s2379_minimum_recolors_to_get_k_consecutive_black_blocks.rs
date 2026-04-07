// Tests for Problem 2379: Minimum Recolors to Get K Consecutive Black Blocks
// Java reference: src/test/java/g2301_2400/s2379_minimum_recolors_to_get_k_consecutive_black_blocks/SolutionTest.java

use leetcode_in_rust::s2379::minimum_recolors_to_get_k_consecutive_black_blocks::Solution;

#[test]
fn test_minimum_recolors() {
    assert_eq!(Solution::minimum_recolors("WBBWWBBWBW".to_string(), 7), 3);
}

#[test]
fn test_minimum_recolors2() {
    assert_eq!(Solution::minimum_recolors("WBWBBBW".to_string(), 2), 0);
}
