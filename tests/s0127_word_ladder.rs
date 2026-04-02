// Tests for Problem 0127: Word Ladder
// Java reference: src/test/java/g0121_0200/s0127_word_ladder/SolutionTest.java

use leetcode_in_rust::s0127::word_ladder::Solution;

#[test]
fn test_ladder_length() {
    let result = Solution::ladder_length(
        String::from("hit"),
        String::from("cog"),
        vec![String::from("hot"), String::from("dot"), String::from("dog"),
             String::from("lot"), String::from("log"), String::from("cog")]
    );
    assert_eq!(result, 5);
}

#[test]
fn test_ladder_length2() {
    let result = Solution::ladder_length(
        String::from("hit"),
        String::from("cog"),
        vec![String::from("hot"), String::from("dot"), String::from("dog"),
             String::from("lot"), String::from("log")]
    );
    assert_eq!(result, 0);
}
