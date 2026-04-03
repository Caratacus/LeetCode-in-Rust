// Tests for Problem 0744: Find Smallest Letter Greater Than Target
// Java reference: src/test/java/g0701_0800/s0744_find_smallest_letter_greater_than_target/SolutionTest.java

use leetcode_in_rust::s0744::find_smallest_letter_greater_than_target::Solution;

#[test]
fn test_next_greatest_letter() {
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'), 'c');
}

#[test]
fn test_next_greatest_letter2() {
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'), 'f');
}

#[test]
fn test_next_greatest_letter3() {
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'), 'f');
}

#[test]
fn test_next_greatest_letter4() {
    assert_eq!(Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'k'), 'c');
}
