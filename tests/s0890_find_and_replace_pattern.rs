// Tests for Problem 0890: Find and Replace Pattern
// Java reference: src/test/java/g0801_0900/s0890_find_and_replace_pattern/SolutionTest.java

use leetcode_in_rust::s0890::find_and_replace_pattern::Solution;

#[test]
fn test_find_and_replace_pattern() {
    assert_eq!(
        Solution::find_and_replace_pattern(
            vec!["abc".to_string(), "deq".to_string(), "mee".to_string(), "aqq".to_string(), "dkd".to_string(), "ccc".to_string()],
            "abb".to_string()
        ),
        vec!["mee".to_string(), "aqq".to_string()]
    );
}

#[test]
fn test_find_and_replace_pattern2() {
    assert_eq!(
        Solution::find_and_replace_pattern(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "a".to_string()
        ),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}
