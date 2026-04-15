// Tests for Problem 3433: Count Mentions Per User
// Java reference: src/test/java/g3401_3500/s3433_count_mentions_per_user/SolutionTest.java

use leetcode_in_rust::s3433::count_mentions_per_user::Solution;

#[test]
fn test_count_mentions() {
    assert_eq!(
        Solution::count_mentions(
            2,
            vec![
                vec!["MESSAGE".to_string(), "10".to_string(), "id1 id0".to_string()],
                vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
                vec!["MESSAGE".to_string(), "71".to_string(), "HERE".to_string()],
            ]
        ),
        vec![2, 2]
    );
}

#[test]
fn test_count_mentions2() {
    assert_eq!(
        Solution::count_mentions(
            2,
            vec![
                vec!["MESSAGE".to_string(), "10".to_string(), "id1 id0".to_string()],
                vec!["OFFLINE".to_string(), "11".to_string(), "0".to_string()],
                vec!["MESSAGE".to_string(), "12".to_string(), "ALL".to_string()],
            ]
        ),
        vec![2, 2]
    );
}

#[test]
fn test_count_mentions3() {
    assert_eq!(
        Solution::count_mentions(
            2,
            vec![
                vec!["OFFLINE".to_string(), "10".to_string(), "0".to_string()],
                vec!["MESSAGE".to_string(), "12".to_string(), "HERE".to_string()],
            ]
        ),
        vec![0, 1]
    );
}
