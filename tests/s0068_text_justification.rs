// Tests for Problem 0068: Text Justification
// Java reference: src/test/java/g0001_0100/s0068_text_justification/SolutionTest.java

use leetcode_in_rust::s0068::text_justification::Solution;

#[test]
fn test_full_justify() {
    let words = vec![
        "This".to_string(), "is".to_string(), "an".to_string(),
        "example".to_string(), "of".to_string(), "text".to_string(),
        "justification.".to_string(),
    ];
    let result = Solution::full_justify(words, 16);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].len(), 16);
    assert_eq!(result[1].len(), 16);
    assert_eq!(result[2].len(), 14);
}

#[test]
fn test_full_justify2() {
    let words = vec![
        "What".to_string(), "must".to_string(), "be".to_string(),
        "acknowledgment".to_string(), "shall".to_string(), "be".to_string(),
    ];
    let result = Solution::full_justify(words, 16);
    assert_eq!(result.len(), 3);
}
