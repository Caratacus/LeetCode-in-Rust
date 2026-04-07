// Tests for Problem 2367: Number of Arithmetic Triplets
// Java reference: src/test/java/g2301_2400/s2367_number_of_arithmetic_triplets/SolutionTest.java

use leetcode_in_rust::s2367::number_of_arithmetic_triplets::Solution;

#[test]
fn test_arithmetic_triplets() {
    assert_eq!(Solution::arithmetic_triplets(vec![0, 1, 4, 6, 7, 10], 3), 2);
}

#[test]
fn test_arithmetic_triplets2() {
    assert_eq!(Solution::arithmetic_triplets(vec![4, 5, 6, 7, 8, 9], 2), 2);
}
