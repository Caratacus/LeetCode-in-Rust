// Tests for Problem 2134: Minimum Swaps to Group All 1's Together II
// Java reference: src/test/java/g2101_2200/s2134_minimum_swaps_to_group_all_1s_together_ii/SolutionTest.java

use leetcode_in_rust::s2134::minimum_swaps_to_group_all_1s_together_ii::Solution;

#[test]
fn test_min_swaps() {
    assert_eq!(Solution::min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
}

#[test]
fn test_min_swaps2() {
    assert_eq!(Solution::min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
}

#[test]
fn test_min_swaps3() {
    assert_eq!(Solution::min_swaps(vec![1, 1, 0, 0, 1]), 0);
}
