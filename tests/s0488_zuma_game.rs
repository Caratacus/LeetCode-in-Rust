// Tests for Problem 0488: Zuma Game
// Java reference: src/test/java/g0401_0500/s0488_zuma_game/SolutionTest.java

use leetcode_in_rust::s0488::zuma_game::Solution;

#[test]
fn test_find_min_step() {
    assert_eq!(Solution::find_min_step("WRRBBW".to_string(), "RB".to_string()), -1);
}

#[test]
fn test_find_min_step2() {
    assert_eq!(
        Solution::find_min_step("WWRRBBWW".to_string(), "WRBRW".to_string()),
        2
    );
}

#[test]
fn test_find_min_step3() {
    assert_eq!(Solution::find_min_step("G".to_string(), "GGGGG".to_string()), 2);
}
