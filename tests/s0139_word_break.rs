// Tests for Problem 0139: Word Break
// Java reference: src/test/java/g0121_0200/s0139_word_break/SolutionTest.java

use leetcode_in_rust::s0139::word_break::Solution;

#[test]
fn test_word_break() {
    assert_eq!(Solution::word_break(
        String::from("leetcode"),
        vec![String::from("leet"), String::from("code")]
    ), true);
}

#[test]
fn test_word_break2() {
    assert_eq!(Solution::word_break(
        String::from("applepenapple"),
        vec![String::from("apple"), String::from("pen")]
    ), true);
}

#[test]
fn test_word_break3() {
    assert_eq!(Solution::word_break(
        String::from("catsandog"),
        vec![String::from("cats"), String::from("dog"), String::from("sand"),
             String::from("and"), String::from("cat")]
    ), false);
}
