// Tests for Problem 3435: Frequencies of Shortest Supersequences
// Java reference: src/test/java/g3401_3500/s3435_frequencies_of_shortest_supersequences/SolutionTest.java

use leetcode_in_rust::s3435::frequencies_of_shortest_supersequences::Solution;

#[test]
fn test_supersequences() {
    assert_eq!(
        Solution::supersequences(vec!["ab".to_string(), "ba".to_string()]),
        vec![
            vec![1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    );
}

#[test]
fn test_supersequences2() {
    assert_eq!(
        Solution::supersequences(vec!["aa".to_string(), "ac".to_string()]),
        vec![vec![2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
    );
}

#[test]
fn test_supersequences3() {
    assert_eq!(
        Solution::supersequences(vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]),
        vec![vec![2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]
    );
}
