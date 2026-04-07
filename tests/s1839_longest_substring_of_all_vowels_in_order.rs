// Tests for Problem 1839: Longest Substring Of All Vowels in Order
// Java reference: src/test/java/g1801_1900/s1839_longest_substring_of_all_vowels_in_order/SolutionTest.java

use leetcode_in_rust::s1839::longest_substring_of_all_vowels_in_order::Solution;

#[test]
fn test_longest_beautiful_substring() {
    assert_eq!(
        Solution::longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string()),
        13
    );
}

#[test]
fn test_longest_beautiful_substring2() {
    assert_eq!(
        Solution::longest_beautiful_substring("aeeeiiiioooauuuaeiou".to_string()),
        5
    );
}

#[test]
fn test_longest_beautiful_substring3() {
    assert_eq!(Solution::longest_beautiful_substring("a".to_string()), 0);
}
