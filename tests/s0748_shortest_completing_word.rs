// Tests for Problem 0748: Shortest Completing Word
// Java reference: src/test/java/g0701_0800/s0748_shortest_completing_word/SolutionTest.java

use leetcode_in_rust::s0748::shortest_completing_word::Solution;

#[test]
fn test_shortest_completing_word() {
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 PSt".to_string(),
            vec![
                "step".to_string(),
                "steps".to_string(),
                "stripe".to_string(),
                "stepple".to_string()
            ]
        ),
        "steps"
    );
}

#[test]
fn test_shortest_completing_word2() {
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 456".to_string(),
            vec![
                "looks".to_string(),
                "pest".to_string(),
                "stew".to_string(),
                "show".to_string()
            ]
        ),
        "pest"
    );
}
