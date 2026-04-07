// Tests for Problem 1982: Find Array Given Subset Sums
// Java reference: src/test/java/g1901_2000/s1982_find_array_given_subset_sums/SolutionTest.java

use leetcode_in_rust::s1982::find_array_given_subset_sums::Solution;

#[test]
fn test_recover_array() {
    assert_eq!(
        Solution::recover_array(3, vec![-3, -2, -1, 0, 0, 1, 2, 3]),
        vec![1, 2, -3]
    );
}

#[test]
fn test_recover_array2() {
    assert_eq!(Solution::recover_array(2, vec![0, 0, 0, 0]), vec![0, 0]);
}

#[test]
fn test_recover_array3() {
    assert_eq!(
        Solution::recover_array(4, vec![0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8]),
        vec![0, -1, 4, 5]
    );
}
