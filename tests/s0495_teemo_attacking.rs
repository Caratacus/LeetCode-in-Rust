// Tests for Problem 0495: Teemo Attacking
// Java reference: src/test/java/g0401_0500/s0495_teemo_attacking/SolutionTest.java

use leetcode_in_rust::s0495::teemo_attacking::Solution;

#[test]
fn test_find_poisoned_duration() {
    assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
}

#[test]
fn test_find_poisoned_duration2() {
    assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
}
