// Tests for Problem 2942: Find Words Containing Character
// Java reference: src/test/java/g2901_3000/s2942_find_words_containing_character/SolutionTest.java

use leetcode_in_rust::s2942::find_words_containing_character::Solution;

#[test]
fn test_find_words_containing() {
    assert_eq!(
        Solution::find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
        vec![0, 1]
    );
}

#[test]
fn test_find_words_containing2() {
    assert_eq!(
        Solution::find_words_containing(
            vec!["abc".to_string(), "bcd".to_string(), "aaaa".to_string(), "cbc".to_string()],
            'a'
        ),
        vec![0, 2]
    );
}

#[test]
fn test_find_words_containing3() {
    assert_eq!(
        Solution::find_words_containing(
            vec!["abc".to_string(), "bcd".to_string(), "aaaa".to_string(), "cbc".to_string()],
            'z'
        ),
        vec![] as Vec<i32>
    );
}
