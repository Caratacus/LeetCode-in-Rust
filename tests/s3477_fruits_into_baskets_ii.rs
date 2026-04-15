// Tests for Problem 3477: Fruits Into Baskets II
// Java reference: src/test/java/g3401_3500/s3477_fruits_into_baskets_ii/SolutionTest.java

use leetcode_in_rust::s3477::fruits_into_baskets_ii::Solution;

#[test]
fn test_num_of_unplaced_fruits() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
}

#[test]
fn test_num_of_unplaced_fruits2() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
}
