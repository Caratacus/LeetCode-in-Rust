// Tests for Problem 3031: Minimum Time to Revert Word to Initial State II
// Java reference: src/test/java/g3001_3100/s3031_minimum_time_to_revert_word_to_initial_state_ii/SolutionTest.java
use leetcode_in_rust::s3031::minimum_time_to_revert_word_to_initial_state_ii::Solution;
#[test]
fn test_minimum_time_to_initial_state() {
    assert_eq!(Solution::minimum_time_to_initial_state(String::from("abacaba"), 3), 2);
}
#[test]
fn test_minimum_time_to_initial_state2() {
    assert_eq!(Solution::minimum_time_to_initial_state(String::from("abacaba"), 4), 1);
}
#[test]
fn test_minimum_time_to_initial_state3() {
    assert_eq!(Solution::minimum_time_to_initial_state(String::from("abcbabcd"), 2), 4);
}
