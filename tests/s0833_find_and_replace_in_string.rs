// Tests for Problem 0833: Find And Replace in String
// Java reference: src/test/java/g0801_0900/s0833_find_and_replace_in_string/SolutionTest.java

use leetcode_in_rust::s0833::find_and_replace_in_string::Solution;

#[test]
fn test_find_replace_string() {
    assert_eq!(
        Solution::find_replace_string(
            "abcd".to_string(),
            vec![0, 2],
            vec!["a".to_string(), "cd".to_string()],
            vec!["eee".to_string(), "ffff".to_string()]
        ),
        "eeebffff".to_string()
    );
}

#[test]
fn test_find_replace_string2() {
    assert_eq!(
        Solution::find_replace_string(
            "abcd".to_string(),
            vec![0, 2],
            vec!["a".to_string(), "cd".to_string()],
            vec!["eee".to_string(), "ffff".to_string()]
        ),
        "eeebffff".to_string()
    );
}
