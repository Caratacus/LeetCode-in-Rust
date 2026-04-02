// Tests for Problem 0557: Reverse Words in a String III
// Java reference: src/test/java/g0501_0600/s0557_reverse_words_in_a_string_iii/SolutionTest.java

use leetcode_in_rust::s0557::reverse_words_in_a_string_iii::Solution;

#[test]
fn test_reverse_words() {
    assert_eq!(
        Solution::reverse_words("Let's take LeetCode contest".to_string()),
        "s'teL ekat edoCteeL tsetnoc"
    );
}

#[test]
fn test_reverse_words2() {
    assert_eq!(Solution::reverse_words("God Ding".to_string()), "doG gniD");
}
