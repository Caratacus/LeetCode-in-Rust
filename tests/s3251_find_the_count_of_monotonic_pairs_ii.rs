// Tests for Problem 3251: Find the Count of Monotonic Pairs II
// Java reference: src/test/java/g3201_3300/s3251_find_the_count_of_monotonic_pairs_ii/SolutionTest.java

use leetcode_in_rust::s3251::find_the_count_of_monotonic_pairs_ii::Solution;

#[test]
fn test_count_of_pairs() {
    assert_eq!(Solution::count_of_pairs(vec![2, 3, 2]), 4);
}

#[test]
fn test_count_of_pairs2() {
    assert_eq!(Solution::count_of_pairs(vec![5, 5, 5, 5]), 126);
}
