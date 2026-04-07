// Tests for Problem 2456: Most Popular Video Creator
// Java reference: src/test/java/g2401_2500/s2456_most_popular_video_creator/SolutionTest.java

use leetcode_in_rust::s2456::most_popular_video_creator::Solution;

#[test]
fn test_most_popular_creator() {
    let result = Solution::most_popular_creator(
        vec!["alice".to_string(), "bob".to_string(), "alice".to_string(), "chris".to_string()],
        vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string()],
        vec![5, 10, 5, 4],
    );
    assert_eq!(result, vec![vec!["alice".to_string(), "one".to_string()]]);
}

#[test]
fn test_most_popular_creator2() {
    let result = Solution::most_popular_creator(
        vec!["alice".to_string(), "alice".to_string(), "alice".to_string()],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec![1, 2, 2],
    );
    assert_eq!(result, vec![vec!["alice".to_string(), "a".to_string()]]);
}
