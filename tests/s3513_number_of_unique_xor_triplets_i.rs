// Tests for Problem 3513: Number of Unique XOR Triplets I
// Java reference: src/test/java/g3501_3600/s3513_number_of_unique_xor_triplets_i/SolutionTest.java

use leetcode_in_rust::s3513::number_of_unique_xor_triplets_i::Solution;

#[test]
fn test_unique_xor_triplets() {
    assert_eq!(Solution::unique_xor_triplets(vec![1, 2]), 2);
}

#[test]
fn test_unique_xor_triplets2() {
    assert_eq!(Solution::unique_xor_triplets(vec![3, 1, 2]), 4);
}
