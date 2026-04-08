// Tests for Problem 3331: Find Subtree Sizes After Changes
// Java reference: src/test/java/g3301_3400/s3331_find_subtree_sizes_after_changes/SolutionTest.java

use leetcode_in_rust::s3331::find_subtree_sizes_after_changes::Solution;

#[test]
fn test_find_subtree_sizes() {
    assert_eq!(
        Solution::find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".to_string()),
        vec![6, 3, 1, 1, 1, 1]
    );
}

#[test]
fn test_find_subtree_sizes2() {
    assert_eq!(
        Solution::find_subtree_sizes(vec![-1, 0, 4, 0, 1], "abbba".to_string()),
        vec![5, 2, 1, 1, 1]
    );
}
