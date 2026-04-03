// Tests for Problem 0960: Delete Columns to Make Sorted III
// Java reference: src/test/java/g0901_1000/s0960_delete_columns_to_make_sorted_iii/SolutionTest.java

use leetcode_in_rust::s0960::delete_columns_to_make_sorted_iii::Solution;

#[test]
fn test_min_deletion_size() {
    let result = Solution::min_deletion_size(vec!["babca".to_string(), "bbazb".to_string()]);
    assert_eq!(result, 3);
}

#[test]
fn test_min_deletion_size2() {
    let result = Solution::min_deletion_size(vec!["edcba".to_string()]);
    assert_eq!(result, 4);
}

#[test]
fn test_min_deletion_size3() {
    let result = Solution::min_deletion_size(vec!["ghi".to_string(), "def".to_string(), "abc".to_string()]);
    assert_eq!(result, 0);
}
