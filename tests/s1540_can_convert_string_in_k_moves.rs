// Tests for Problem 1540: Can Convert String in K Moves
// Java reference: src/test/java/g1501_1600/s1540_can_convert_string_in_k_moves/SolutionTest.java

use leetcode_in_rust::s1540::can_convert_string_in_k_moves::Solution;

#[test]
fn test_can_convert_string() {
    assert_eq!(Solution::can_convert_string("input".to_string(), "ouput".to_string(), 9), true);
}

#[test]
fn test_can_convert_string2() {
    assert_eq!(Solution::can_convert_string("abc".to_string(), "bcd".to_string(), 10), false);
}

#[test]
fn test_can_convert_string3() {
    assert_eq!(Solution::can_convert_string("aab".to_string(), "bbb".to_string(), 27), true);
}
