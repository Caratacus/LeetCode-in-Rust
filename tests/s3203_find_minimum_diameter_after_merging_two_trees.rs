// Tests for Problem 3203: Find Minimum Diameter After Merging Two Trees
// Java reference: src/test/java/g3201_3300/s3203_find_minimum_diameter_after_merging_two_trees/SolutionTest.java

use leetcode_in_rust::s3203::find_minimum_diameter_after_merging_two_trees::Solution;

#[test]
fn test_minimum_diameter_after_merge() {
    assert_eq!(
        Solution::minimum_diameter_after_merge(
            vec![vec![0, 1], vec![0, 2], vec![0, 3]],
            vec![vec![0, 1]]
        ),
        3
    );
}

#[test]
fn test_minimum_diameter_after_merge2() {
    assert_eq!(
        Solution::minimum_diameter_after_merge(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]],
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![2, 5], vec![3, 6], vec![2, 7]]
        ),
        5
    );
}
