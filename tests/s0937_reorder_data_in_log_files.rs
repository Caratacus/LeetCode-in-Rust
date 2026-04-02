// Tests for Problem 0937: Reorder Data in Log Files
// Java reference: src/test/java/g0901_1000/s0937_reorder_data_in_log_files/SolutionTest.java

use leetcode_in_rust::s0937::reorder_data_in_log_files::Solution;

#[test]
fn test_reorder_log_files() {
    assert_eq!(
        Solution::reorder_log_files(vec![
            "dig1 8 1 5 1".to_string(),
            "let1 art can".to_string(),
            "dig2 3 6".to_string(),
            "let2 own kit dig".to_string(),
            "let3 art zero".to_string()
        ]),
        vec![
            "let1 art can".to_string(),
            "let3 art zero".to_string(),
            "let2 own kit dig".to_string(),
            "dig1 8 1 5 1".to_string(),
            "dig2 3 6".to_string()
        ]
    );
}

#[test]
fn test_reorder_log_files2() {
    assert_eq!(
        Solution::reorder_log_files(vec![
            "a1 9 2 3 1".to_string(),
            "g1 act car".to_string(),
            "zo4 4 7".to_string(),
            "ab1 off key dog".to_string(),
            "a8 act zoo".to_string()
        ]),
        vec![
            "g1 act car".to_string(),
            "a8 act zoo".to_string(),
            "ab1 off key dog".to_string(),
            "a1 9 2 3 1".to_string(),
            "zo4 4 7".to_string()
        ]
    );
}
