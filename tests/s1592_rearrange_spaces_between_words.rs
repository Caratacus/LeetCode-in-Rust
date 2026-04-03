// Tests for Problem 1592: Rearrange Spaces Between Words
// Java reference: src/test/java/g1501_1600/s1592_rearrange_spaces_between_words/SolutionTest.java

use leetcode_in_rust::s1592::rearrange_spaces_between_words::Solution;

#[test]
fn test_reorder_spaces() {
    assert_eq!(
        Solution::reorder_spaces("  this   is  a sentence ".to_string()),
        "this   is   a   sentence"
    );
}

#[test]
fn test_reorder_spaces2() {
    assert_eq!(
        Solution::reorder_spaces(" practice   makes   perfect".to_string()),
        "practice   makes   perfect "
    );
}
