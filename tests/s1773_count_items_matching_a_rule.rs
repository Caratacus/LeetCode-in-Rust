// Tests for Problem 1773: Count Items Matching a Rule
// Java reference: src/test/java/g1701_1800/s1773_count_items_matching_a_rule/SolutionTest.java

use leetcode_in_rust::s1773::count_items_matching_a_rule::Solution;

#[test]
fn test_count_matches() {
    assert_eq!(
        Solution::count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec!["computer".to_string(), "silver".to_string(), "lenovo".to_string()],
                vec!["phone".to_string(), "gold".to_string(), "iphone".to_string()],
            ],
            "color",
            "silver"
        ),
        1
    );
}

#[test]
fn test_count_matches2() {
    assert_eq!(
        Solution::count_matches(
            vec![
                vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
                vec!["computer".to_string(), "silver".to_string(), "phone".to_string()],
                vec!["phone".to_string(), "gold".to_string(), "iphone".to_string()],
            ],
            "type",
            "phone"
        ),
        2
    );
}
