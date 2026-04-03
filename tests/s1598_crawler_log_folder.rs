// Tests for Problem 1598: Crawler Log Folder
// Java reference: src/test/java/g1501_1600/s1598_crawler_log_folder/SolutionTest.java

use leetcode_in_rust::s1598::crawler_log_folder::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "../".to_string(),
            "d21/".to_string(),
            "./".to_string()
        ]),
        2
    );
}

#[test]
fn test_min_operations2() {
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "d2/".to_string(),
            "./".to_string(),
            "d3/".to_string(),
            "../".to_string(),
            "d31/".to_string()
        ]),
        3
    );
}

#[test]
fn test_min_operations3() {
    assert_eq!(
        Solution::min_operations(vec![
            "d1/".to_string(),
            "../".to_string(),
            "../".to_string(),
            "../".to_string()
        ]),
        0
    );
}
