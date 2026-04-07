// Tests for Problem 1899: Merge Triplets to Form Target Triplet
// Java reference: src/test/java/g1801_1900/s1899_merge_triplets_to_form_target_triplet/SolutionTest.java

use leetcode_in_rust::s1899::merge_triplets_to_form_target_triplet::Solution;

#[test]
fn test_merge_triplets() {
    assert_eq!(
        Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
            vec![2, 7, 5]
        ),
        true
    );
}

#[test]
fn test_merge_triplets2() {
    assert_eq!(
        Solution::merge_triplets(vec![vec![3, 4, 5], vec![4, 5, 6]], vec![3, 2, 5]),
        false
    );
}

#[test]
fn test_merge_triplets3() {
    assert_eq!(
        Solution::merge_triplets(
            vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]],
            vec![5, 5, 5]
        ),
        true
    );
}
