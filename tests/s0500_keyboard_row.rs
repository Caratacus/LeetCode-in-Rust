// Tests for Problem 0500: Keyboard Row
// Java reference: src/test/java/g0401_0500/s0500_keyboard_row/SolutionTest.java

use leetcode_in_rust::s0500::keyboard_row::Solution;

#[test]
fn test_find_words() {
    let result = Solution::find_words(vec![
        "Hello".to_string(),
        "Alaska".to_string(),
        "Dad".to_string(),
        "Peace".to_string(),
    ]);
    assert_eq!(result, vec!["Alaska".to_string(), "Dad".to_string()]);
}

#[test]
fn test_find_words2() {
    let result = Solution::find_words(vec!["omk".to_string()]);
    assert!(result.is_empty());
}

#[test]
fn test_find_words3() {
    let result = Solution::find_words(vec!["adsdf".to_string(), "sfd".to_string()]);
    assert_eq!(result, vec!["adsdf".to_string(), "sfd".to_string()]);
}
