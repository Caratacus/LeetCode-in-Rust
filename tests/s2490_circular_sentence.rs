// Tests for Problem 2490: Circular Sentence
// Java reference: src/test/java/g2401_2500/s2490_circular_sentence/SolutionTest.java

use leetcode_in_rust::s2490::circular_sentence::Solution;

#[test]
fn test_is_circular_sentence() {
    assert_eq!(Solution::is_circular_sentence("leetcode exercises sound delightful".to_string()), true);
}

#[test]
fn test_is_circular_sentence2() {
    assert_eq!(Solution::is_circular_sentence("eetcode".to_string()), true);
}
