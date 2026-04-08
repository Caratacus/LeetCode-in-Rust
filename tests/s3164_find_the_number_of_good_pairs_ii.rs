// Tests for Problem 3164: Find the Number of Good Pairs II
// Java reference: src/test/java/g3101_3200/s3164_find_the_number_of_good_pairs_ii/SolutionTest.java

use leetcode_in_rust::s3164::find_the_number_of_good_pairs_ii::Solution;
#[test]
fn test_number_of_pairs() {
    assert_eq!(Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1), 5);
}
#[test]
fn test_number_of_pairs2() {
    assert_eq!(Solution::number_of_pairs(vec![1, 2, 4, 12], vec![2, 4, 3], 2);
}
