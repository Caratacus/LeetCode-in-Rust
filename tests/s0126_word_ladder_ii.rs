// Tests for Problem 0126: Word Ladder II
// Java reference: src/test/java/g0121_0200/s0126_word_ladder_ii/SolutionTest.java

use leetcode_in_rust::s0126::word_ladder_ii::Solution;

#[test]
fn test_find_ladders() {
    let result = Solution::find_ladders(
        String::from("hit"),
        String::from("cog"),
        vec![String::from("hot"), String::from("dot"), String::from("dog"),
             String::from("lot"), String::from("log"), String::from("cog")]
    );
    assert!(result.len() > 0);
}
