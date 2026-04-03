// Tests for Problem 0844: Backspace String Compare
// Java reference: src/test/java/g0801_0900/s0844_backspace_string_compare/SolutionTest.java

use leetcode_in_rust::s0844::backspace_string_compare::Solution;

#[test]
fn test_backspace_compare() {
    assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
}

#[test]
fn test_backspace_compare2() {
    assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
}

#[test]
fn test_backspace_compare3() {
    assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
}
