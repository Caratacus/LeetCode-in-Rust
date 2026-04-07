// Tests for Problem 2475: Number of Unequal Triplets in Array
// Java reference: src/test/java/g2401_2500/s2475_number_of_unequal_triplets_in_array/SolutionTest.java

use leetcode_in_rust::s2475::number_of_unequal_triplets_in_array::Solution;

#[test]
fn test_unequal_triplets() {
    assert_eq!(Solution::unequal_triplets(vec![4, 4, 2, 4, 3]), 3);
}

#[test]
fn test_unequal_triplets2() {
    assert_eq!(Solution::unequal_triplets(vec![1, 1, 1, 1, 1]), 0);
}
