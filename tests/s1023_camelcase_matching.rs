// Tests for Problem 1023: Camelcase Matching
// Java reference: src/test/java/g1001_1100/s1023_camelcase_matching/SolutionTest.java

use leetcode_in_rust::s1023::camelcase_matching::Solution;

#[test]
fn test_camel_match() {
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FB".to_string()
        ),
        vec![true, false, true, true, false]
    );
}

#[test]
fn test_camel_match2() {
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FoBa".to_string()
        ),
        vec![true, false, true, false, false]
    );
}

#[test]
fn test_camel_match3() {
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FoBaT".to_string()
        ),
        vec![false, true, false, false, false]
    );
}
