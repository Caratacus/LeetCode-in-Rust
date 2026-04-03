// Tests for Problem 0682: Baseball Game
// Java reference: src/test/java/g0601_0700/s0682_baseball_game/SolutionTest.java

use leetcode_in_rust::s0682::baseball_game::Solution;

#[test]
fn test_cal_points() {
    assert_eq!(
        Solution::cal_points(vec!["5".to_string(), "2".to_string(), "C".to_string(), "D".to_string(), "+".to_string()]),
        30
    );
}

#[test]
fn test_cal_points2() {
    assert_eq!(
        Solution::cal_points(vec![
            "5".to_string(), "-2".to_string(), "4".to_string(), "C".to_string(),
            "D".to_string(), "9".to_string(), "+".to_string(), "+".to_string()
        ]),
        27
    );
}

#[test]
fn test_cal_points3() {
    assert_eq!(Solution::cal_points(vec!["1".to_string()]), 1);
}
