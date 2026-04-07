// Tests for Problem 2707: Extra Characters in a String
// Java reference: src/test/java/g2701_2800/s2707_extra_characters_in_a_string/SolutionTest.java

use leetcode_in_rust::s2707::extra_characters_in_a_string::Solution;

#[test]
fn test_min_extra_char() {
    assert_eq!(
        Solution::min_extra_char(
            "leetscode".to_string(),
            vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()]
        ),
        1
    );
}

#[test]
fn test_min_extra_char2() {
    assert_eq!(
        Solution::min_extra_char(
            "sayhelloworld".to_string(),
            vec!["hello".to_string(), "world".to_string()]
        ),
        3
    );
}
