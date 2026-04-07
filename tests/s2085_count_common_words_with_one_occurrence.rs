// Tests for Problem 2085: Count Common Words With One Occurrence
// Java reference: src/test/java/g2001_2100/s2085_count_common_words_with_one_occurrence/SolutionTest.java

use leetcode_in_rust::s2085::count_common_words_with_one_occurrence::Solution;

#[test]
fn test_count_words() {
    assert_eq!(
        Solution::count_words(
            vec![
                "leetcode".to_string(),
                "is".to_string(),
                "amazing".to_string(),
                "as".to_string(),
                "is".to_string()
            ],
            vec![
                "amazing".to_string(),
                "leetcode".to_string(),
                "is".to_string()
            ]
        ),
        2
    );
}

#[test]
fn test_count_words2() {
    assert_eq!(
        Solution::count_words(
            vec!["b".to_string(), "bb".to_string(), "bbb".to_string()],
            vec!["a".to_string(), "aa".to_string(), "aaa".to_string()]
        ),
        0
    );
}

#[test]
fn test_count_words3() {
    assert_eq!(
        Solution::count_words(
            vec!["a".to_string(), "ab".to_string()],
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "ab".to_string()]
        ),
        1
    );
}
