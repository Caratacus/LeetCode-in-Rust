// Tests for Problem 0299: Bulls and Cows
// Java reference: src/test/java/g0201_0300/s0299_bulls_and_cows/SolutionTest.java

use leetcode_in_rust::s0299::bulls_and_cows::Solution;

#[test]
fn test_get_hint() {
    assert_eq!(Solution::get_hint("1807".to_string(), "7810".to_string()), "1A3B");
}

#[test]
fn test_get_hint2() {
    assert_eq!(Solution::get_hint("1123".to_string(), "0111".to_string()), "1A1B");
}

#[test]
fn test_get_hint3() {
    assert_eq!(Solution::get_hint("1".to_string(), "0".to_string()), "0A0B");
}

#[test]
fn test_get_hint4() {
    assert_eq!(Solution::get_hint("1".to_string(), "1".to_string()), "1A0B");
}
