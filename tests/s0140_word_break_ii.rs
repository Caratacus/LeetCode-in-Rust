// Tests for Problem 0140: Word Break II
// Java reference: src/test/java/g0121_0200/s0140_word_break_ii/SolutionTest.java

use leetcode_in_rust::s0140::word_break_ii::Solution;

#[test]
fn test_word_break() {
    let result = Solution::word_break(
        String::from("catsanddog"),
        vec![String::from("cat"), String::from("cats"), String::from("and"),
             String::from("sand"), String::from("dog")]
    );
    assert!(result.contains(&String::from("cats and dog")));
    assert!(result.contains(&String::from("cat sand dog")));
}
