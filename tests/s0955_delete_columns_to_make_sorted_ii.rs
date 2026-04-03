// Tests for Problem 0955: Delete Columns to Make Sorted II
// Java reference: src/test/java/g0901_1000/s0955_delete_columns_to_make_sorted_ii/SolutionTest.java

use leetcode_in_rust::s0955::delete_columns_to_make_sorted_ii::Solution;

#[test]
fn test_min_deletion_size() {
    let result = Solution::min_deletion_size(vec!["ca".to_string(), "bb".to_string(), "ac".to_string()]);
    assert_eq!(result, 1);
}

#[test]
fn test_min_deletion_size2() {
    let result = Solution::min_deletion_size(vec!["xc".to_string(), "yb".to_string(), "za".to_string()]);
    assert_eq!(result, 0);
}

#[test]
fn test_min_deletion_size3() {
    let result = Solution::min_deletion_size(vec!["zyx".to_string(), "wvu".to_string(), "tsr".to_string()]);
    assert_eq!(result, 3);
}
