// Tests for Problem 2770: Maximum Number of Jumps to Reach the Last Index
// Java reference: src/test/java/g2701_2800/s2770_maximum_number_of_jumps_to_reach_the_last_index/SolutionTest.java

use leetcode_in_rust::s2770::maximum_number_of_jumps_to_reach_the_last_index::Solution;

#[test]
fn test_maximum_jumps() {
    assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 2), 3);
}

#[test]
fn test_maximum_jumps2() {
    assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 3), 5);
}

#[test]
fn test_maximum_jumps3() {
    assert_eq!(Solution::maximum_jumps(vec![1, 3, 6, 4, 1, 2], 0), -1);
}
