// Tests for Problem 2191: Sort the Jumbled Numbers
// Java reference: src/test/java/g2101_2200/s2191_sort_the_jumbled_numbers/SolutionTest.java

use leetcode_in_rust::s2191::sort_the_jumbled_numbers::Solution;

#[test]
fn test_sort_jumbled() {
    assert_eq!(
        Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
        vec![338, 38, 991]
    );
}

#[test]
fn test_sort_jumbled2() {
    assert_eq!(
        Solution::sort_jumbled(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![789, 456, 123]),
        vec![123, 456, 789]
    );
}
