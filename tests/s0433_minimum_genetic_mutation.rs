// Tests for Problem 0433: Minimum Genetic Mutation
// Java reference: src/test/java/g0401_0500/s0433_minimum_genetic_mutation/SolutionTest.java

use leetcode_in_rust::s0433::minimum_genetic_mutation::Solution;

#[test]
fn test_min_mutation() {
    assert_eq!(
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()]
        ),
        1
    );
}

#[test]
fn test_min_mutation2() {
    assert_eq!(
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AAACGGTA".to_string(),
            vec![
                "AACCGGTA".to_string(),
                "AACCGCTA".to_string(),
                "AAACGGTA".to_string()
            ]
        ),
        2
    );
}

#[test]
fn test_min_mutation3() {
    assert_eq!(
        Solution::min_mutation(
            "AAAAACCC".to_string(),
            "AACCCCCC".to_string(),
            vec![
                "AAAACCCC".to_string(),
                "AAACCCCC".to_string(),
                "AACCCCCC".to_string()
            ]
        ),
        3
    );
}
