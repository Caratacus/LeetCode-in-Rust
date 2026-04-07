// Tests for Problem 2446: Determine if Two Events Have Conflict
// Java reference: src/test/java/g2401_2500/s2446_determine_if_two_events_have_conflict/SolutionTest.java

use leetcode_in_rust::s2446::determine_if_two_events_have_conflict::Solution;

#[test]
fn test_have_conflict() {
    assert_eq!(
        Solution::have_conflict(
            vec!["01:15".to_string(), "02:00".to_string()],
            vec!["02:00".to_string(), "03:00".to_string()]
        ),
        true
    );
}

#[test]
fn test_have_conflict2() {
    assert_eq!(
        Solution::have_conflict(
            vec!["01:00".to_string(), "02:00".to_string()],
            vec!["01:20".to_string(), "03:00".to_string()]
        ),
        true
    );
}

#[test]
fn test_have_conflict3() {
    assert_eq!(
        Solution::have_conflict(
            vec!["10:00".to_string(), "11:00".to_string()],
            vec!["14:00".to_string(), "15:00".to_string()]
        ),
        false
    );
}
