// Tests for Problem 0792: Number of Matching Subsequences
// Java reference: src/test/java/g0701_0800/s0792_number_of_matching_subsequences/SolutionTest.java

use leetcode_in_rust::s0792::number_of_matching_subsequences::Solution;

#[test]
fn test_num_matching_subseq() {
    assert_eq!(
        Solution::num_matching_subseq(
            "abcde".to_string(),
            vec!["a".to_string(), "bb".to_string(), "acd".to_string(), "ace".to_string()]
        ),
        3
    );
}

#[test]
fn test_num_matching_subseq2() {
    assert_eq!(
        Solution::num_matching_subseq(
            "dsahjpjauf".to_string(),
            vec![
                "ahjpjau".to_string(),
                "ja".to_string(),
                "ahbwzgqnuk".to_string(),
                "tnmlanowax".to_string()
            ]
        ),
        2
    );
}
