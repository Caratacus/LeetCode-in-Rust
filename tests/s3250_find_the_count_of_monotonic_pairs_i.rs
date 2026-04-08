// Tests for Problem 3250: Find the Count of Monotonic Pairs I
// Java reference: src/test/java/g3201_3300/s3250_find_the_count_of_monotonic_pairs_i/SolutionTest.java

use leetcode_in_rust::s3250::find_the_count_of_monotonic_pairs_i::Solution;

#[test]
fn test_count_of_pairs() {
    assert_eq!(Solution::count_of_pairs(vec![2, 3, 2]), 4);
}

#[test]
fn test_count_of_pairs2() {
    assert_eq!(Solution::count_of_pairs(vec![5, 5, 5, 5]), 126);
}
