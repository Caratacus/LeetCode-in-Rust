// Tests for Problem 1125: Smallest Sufficient Team
// Java reference: src/test/java/g1101_1200/s1125_smallest_sufficient_team/SolutionTest.java

use leetcode_in_rust::s1125::smallest_sufficient_team::Solution;

#[test]
fn test_smallest_sufficient_team() {
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec!["java".to_string(), "nodejs".to_string(), "reactjs".to_string()],
            vec![
                vec!["java".to_string()],
                vec!["nodejs".to_string()],
                vec!["nodejs".to_string(), "reactjs".to_string()]
            ]
        ),
        vec![0, 2]
    );
}

#[test]
fn test_smallest_sufficient_team2() {
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec![
                "algorithms".to_string(),
                "math".to_string(),
                "java".to_string(),
                "reactjs".to_string(),
                "csharp".to_string(),
                "aws".to_string()
            ],
            vec![
                vec!["algorithms".to_string(), "math".to_string(), "java".to_string()],
                vec!["algorithms".to_string(), "math".to_string(), "reactjs".to_string()],
                vec!["java".to_string(), "csharp".to_string(), "aws".to_string()],
                vec!["reactjs".to_string(), "csharp".to_string()],
                vec!["csharp".to_string(), "math".to_string()],
                vec!["aws".to_string(), "java".to_string()]
            ]
        ),
        vec![1, 2]
    );
}
