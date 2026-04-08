// Tests for Problem 3162: Find the Number of Good Pairs I
// Java reference: src/test/java/g3101_3200/s3162_find_the_number_of_good_pairs_i/SolutionTest.java

use leetcode_in_rust::s3162::find_the_number_of_good_pairs_i::Solution;
#[test]
fn test_number_of_pairs() {
    assert_eq!(Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
}
#[test]
fn test_number_of_pairs2() {
    assert_eq!(Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4], 3), 2);
}
