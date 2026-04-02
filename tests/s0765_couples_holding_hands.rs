// Tests for Problem 0765: Couples Holding Hands
// Java reference: src/test/java/g0701_0800/s0765_couples_holding_hands/SolutionTest.java

use leetcode_in_rust::s0765::couples_holding_hands::Solution;

#[test]
fn test_min_swaps_couples() {
    assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
}

#[test]
fn test_min_swaps_couples2() {
    assert_eq!(Solution::min_swaps_couples(vec![0, 4, 7, 3, 1, 5, 2, 8, 6, 9]), 3);
}

#[test]
fn test_min_swaps_couples3() {
    assert_eq!(
        Solution::min_swaps_couples(vec![5, 6, 4, 0, 2, 1, 9, 3, 8, 7, 11, 10]),
        4
    );
}
