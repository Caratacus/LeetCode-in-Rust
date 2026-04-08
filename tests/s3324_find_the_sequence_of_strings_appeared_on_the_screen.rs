// Tests for Problem 3324: Find the Sequence of Strings Appeared on the Screen
// Java reference: src/test/java/g3301_3400/s3324_find_the_sequence_of_strings_appeared_on_the_screen/SolutionTest.java

use leetcode_in_rust::s3324::find_the_sequence_of_strings_appeared_on_the_screen::Solution;

#[test]
fn test_string_sequence() {
    assert_eq!(
        Solution::string_sequence("abc".to_string()),
        vec!["a", "aa", "ab", "aba", "abb", "abc"]
    );
}

#[test]
fn test_string_sequence2() {
    assert_eq!(
        Solution::string_sequence("he".to_string()),
        vec!["a", "b", "c", "d", "e", "f", "g", "h", "ha", "hb", "hc", "hd", "he"]
    );
}
