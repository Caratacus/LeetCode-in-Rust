// Tests for Problem 1002: Find Common Characters
// Java reference: src/test/java/g1001_1100/s1002_find_common_characters/SolutionTest.java

use leetcode_in_rust::s1002::find_common_characters::Solution;

#[test]
fn test_common_chars() {
    assert_eq!(
        Solution::common_chars(vec!["bella".to_string(), "label".to_string(), "roller".to_string()]),
        vec!["e".to_string(), "l".to_string(), "l".to_string()]
    );
}

#[test]
fn test_common_chars2() {
    assert_eq!(
        Solution::common_chars(vec!["cool".to_string(), "lock".to_string(), "cook".to_string()]),
        vec!["c".to_string(), "o".to_string()]
    );
}
