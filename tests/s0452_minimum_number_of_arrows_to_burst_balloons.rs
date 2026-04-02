// Tests for Problem 0452: Minimum Number of Arrows to Burst Balloons
// Java reference: src/test/java/g0401_0500/s0452_minimum_number_of_arrows_to_burst_balloons/SolutionTest.java

use leetcode_in_rust::s0452::minimum_number_of_arrows_to_burst_balloons::Solution;

#[test]
fn test_find_min_arrow_shots() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
        2
    );
}

#[test]
fn test_find_min_arrow_shots2() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
        4
    );
}

#[test]
fn test_find_min_arrow_shots3() {
    assert_eq!(
        Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
        2
    );
}
