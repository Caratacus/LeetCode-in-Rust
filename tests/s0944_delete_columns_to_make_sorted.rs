// Tests for Problem 0944: Delete Columns to Make Sorted
// Java reference: src/test/java/g0901_1000/s0944_delete_columns_to_make_sorted/SolutionTest.java

use leetcode_in_rust::s0944::delete_columns_to_make_sorted::Solution;

#[test]
fn test_min_deletion_size() {
    assert_eq!(
        Solution::min_deletion_size(vec!["cba".to_string(), "daf".to_string(), "ghi".to_string()]),
        1
    );
}

#[test]
fn test_min_deletion_size2() {
    assert_eq!(
        Solution::min_deletion_size(vec!["a".to_string(), "b".to_string()]),
        0
    );
}

#[test]
fn test_min_deletion_size3() {
    assert_eq!(
        Solution::min_deletion_size(vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()]),
        3
    );
}
