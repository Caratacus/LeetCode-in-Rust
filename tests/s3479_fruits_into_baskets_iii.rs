// Tests for Problem 3479: Fruits Into Baskets III
// Java reference: src/test/java/g3401_3500/s3479_fruits_into_baskets_iii/SolutionTest.java

use leetcode_in_rust::s3479::fruits_into_baskets_iii::Solution;

#[test]
fn test_num_of_unplaced_fruits() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![4, 2, 5], vec![3, 5, 4]), 1);
}

#[test]
fn test_num_of_unplaced_fruits2() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![3, 6, 1], vec![6, 4, 7]), 0);
}

#[test]
fn test_num_of_unplaced_fruits3() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![1, 2, 3], vec![3, 2, 1]), 1);
}

#[test]
fn test_num_of_unplaced_fruits4() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![4, 5, 6], vec![1, 2, 3]), 3);
}

#[test]
fn test_num_of_unplaced_fruits5() {
    assert_eq!(Solution::num_of_unplaced_fruits(vec![1, 5, 2, 6], vec![2, 3]), 2);
}
