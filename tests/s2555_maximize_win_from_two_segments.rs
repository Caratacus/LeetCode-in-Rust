// Tests for Problem 2555: Maximize Win From Two Segments
// Java reference: src/test/java/g2501_2600/s2555_maximize_win_from_two_segments/SolutionTest.java

use leetcode_in_rust::s2555::maximize_win_from_two_segments::Solution;

#[test]
fn test_maximize_win() {
    assert_eq!(Solution::maximize_win(vec![1, 1, 2, 2, 3, 3, 5], 2), 7);
}

#[test]
fn test_maximize_win2() {
    assert_eq!(Solution::maximize_win(vec![1, 2, 3, 4], 0), 2);
}
