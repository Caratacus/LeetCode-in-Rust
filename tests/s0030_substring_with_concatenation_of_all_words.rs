// Tests for Problem 0030: Substring with Concatenation of All Words
// Java reference: src/test/java/g0001_0100/s0030_substring_with_concatenation_of_all_words/SolutionTest.java

use leetcode_in_rust::s0030::substring_with_concatenation_of_all_words::Solution;
use leetcode_in_rust::utils::array_utils::compare_unsorted;

#[test]
fn test_find_substring() {
    let result = Solution::find_substring(
        "barfoothefoobarman".to_string(),
        vec!["foo".to_string(), "bar".to_string()],
    );
    let expected = vec![0, 9];
    assert!(compare_unsorted(&result, &expected));
}

#[test]
fn test_find_substring2() {
    let result = Solution::find_substring(
        "wordgoodgoodgoodbestword".to_string(),
        vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()],
    );
    let expected: Vec<i32> = vec![];
    assert_eq!(result, expected);
}

#[test]
fn test_find_substring3() {
    let result = Solution::find_substring(
        "barfoofoobarthefoobarman".to_string(),
        vec!["bar".to_string(), "foo".to_string(), "the".to_string()],
    );
    let expected = vec![6, 9, 12];
    assert!(compare_unsorted(&result, &expected));
}
