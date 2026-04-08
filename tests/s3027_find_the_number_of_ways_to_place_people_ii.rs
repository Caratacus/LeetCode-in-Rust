// Tests for Problem 3027: Find the Number of Ways to Place People II
// Java reference: src/test/java/g3001_3100/s3027_find_the_number_of_ways_to_place_people_ii/SolutionTest.java
use leetcode_in_rust::s3027::find_the_number_of_ways_to_place_people_ii::Solution;
#[test]
fn test_number_of_pairs() {
    assert_eq!(Solution::number_of_pairs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 0);
}
#[test]
fn test_number_of_pairs2() {
    assert_eq!(Solution::number_of_pairs(vec![vec![6, 2], vec![4, 4], vec![2, 6]]), 2);
}
#[test]
fn test_number_of_pairs3() {
    assert_eq!(Solution::number_of_pairs(vec![vec![3, 1], vec![1, 3], vec![1, 1]]), 2);
}
