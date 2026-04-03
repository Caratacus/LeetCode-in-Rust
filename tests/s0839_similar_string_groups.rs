// Tests for Problem 0839: Similar String Groups
// Java reference: src/test/java/g0801_0900/s0839_similar_string_groups/SolutionTest.java

use leetcode_in_rust::s0839::similar_string_groups::Solution;

#[test]
fn test_num_similar_groups() {
    assert_eq!(
        Solution::num_similar_groups(vec![
            "tars".to_string(),
            "rats".to_string(),
            "arts".to_string(),
            "star".to_string()
        ]),
        2
    );
}

#[test]
fn test_num_similar_groups2() {
    assert_eq!(
        Solution::num_similar_groups(vec!["omv".to_string(), "ovm".to_string()]),
        1
    );
}
