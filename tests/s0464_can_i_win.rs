// Tests for Problem 0464: Can I Win
// Java reference: src/test/java/g0401_0500/s0464_can_i_win/SolutionTest.java

use leetcode_in_rust::s0464::can_i_win::Solution;

#[test]
fn test_can_i_win() {
    assert_eq!(Solution::can_i_win(10, 0), true);
}

#[test]
fn test_can_i_win2() {
    assert_eq!(Solution::can_i_win(10, 1), true);
}

#[test]
fn test_can_i_win3() {
    assert_eq!(Solution::can_i_win(10, 11), false);
}
