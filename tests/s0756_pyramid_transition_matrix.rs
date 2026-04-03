// Tests for Problem 0756: Pyramid Transition Matrix
// Java reference: src/test/java/g0701_0800/s0756_pyramid_transition_matrix/SolutionTest.java

use leetcode_in_rust::s0756::pyramid_transition_matrix::Solution;

#[test]
fn test_pyramid_transition() {
    assert_eq!(
        Solution::pyramid_transition(
            "BCD".to_string(),
            vec![
                "BCC".to_string(),
                "CDE".to_string(),
                "CEA".to_string(),
                "FFF".to_string()
            ]
        ),
        true
    );
}

#[test]
fn test_pyramid_transition2() {
    assert_eq!(
        Solution::pyramid_transition(
            "AAAA".to_string(),
            vec![
                "AAB".to_string(),
                "AAC".to_string(),
                "BCD".to_string(),
                "BBE".to_string(),
                "DEF".to_string()
            ]
        ),
        false
    );
}
