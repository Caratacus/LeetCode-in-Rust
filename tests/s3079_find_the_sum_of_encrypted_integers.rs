// Tests for Problem 3079: Find the Sum of Encrypted Integers
// Java reference: src/test/java/g3001_3100/s3079_find_the_sum_of_encrypted_integers/SolutionTest.java

use leetcode_in_rust::s3079::find_the_sum_of_encrypted_integers::Solution;

#[test]
fn test_sum_of_encrypted_int() {
    assert_eq!(Solution::sum_of_encrypted_int(vec![1, 2, 3]), 6);
}

#[test]
fn test_sum_of_encrypted_int2() {
    assert_eq!(Solution::sum_of_encrypted_int(vec![10, 21, 31]), 66);
}
