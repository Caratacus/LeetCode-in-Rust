// Tests for Problem 3514: Number of Unique XOR Triplets II
// Java reference: src/test/java/g3501_3600/s3514_number_of_unique_xor_triplets_ii/SolutionTest.java

use leetcode_in_rust::s3514::number_of_unique_xor_triplets_ii::Solution;

#[test]
fn test_unique_xor_triplets() {
    assert_eq!(Solution::unique_xor_triplets(vec![1, 3]), 2);
}

#[test]
fn test_unique_xor_triplets2() {
    assert_eq!(Solution::unique_xor_triplets(vec![6, 7, 8, 9]), 4);
}
