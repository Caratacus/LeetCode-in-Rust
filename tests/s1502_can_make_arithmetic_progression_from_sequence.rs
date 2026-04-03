// Tests for Problem 1502: Can Make Arithmetic Progression From Sequence
// Java reference: src/test/java/g1501_1600/s1502_can_make_arithmetic_progression_from_sequence/SolutionTest.java

use leetcode_in_rust::s1502::can_make_arithmetic_progression_from_sequence::Solution;

#[test]
fn test_can_make_arithmetic_progression() {
    assert_eq!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]), true);
}

#[test]
fn test_can_make_arithmetic_progression2() {
    assert_eq!(Solution::can_make_arithmetic_progression(vec![1, 2, 4]), false);
}
