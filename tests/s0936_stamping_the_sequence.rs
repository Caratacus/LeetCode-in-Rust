// Tests for Problem 0936: Stamping the Sequence
// Java reference: src/test/java/g0901_1000/s0936_stamping_the_sequence/SolutionTest.java

use leetcode_in_rust::s0936::stamping_the_sequence::Solution;

#[test]
fn test_moves_to_stamp() {
    let result = Solution::moves_to_stamp("abc".to_string(), "ababc".to_string());
    // Result can be [0, 2] or [2, 0]
    assert!(result.contains(&0) && result.contains(&2));
}

#[test]
fn test_moves_to_stamp2() {
    let result = Solution::moves_to_stamp("abca".to_string(), "aabcaca".to_string());
    // Result can be [3, 0, 1] or other valid order
    assert!(result.contains(&0) && result.contains(&1) && result.contains(&3));
}
