// Tests for Problem 2909: Minimum Sum of Mountain Triplets II
// Java reference: src/test/java/g2901_3000/s2909_minimum_sum_of_mountain_triplets_ii/SolutionTest.java

use leetcode_in_rust::s2909::minimum_sum_of_mountain_triplets_ii::Solution;

#[test]
fn test_minimum_sum() {
    assert_eq!(Solution::minimum_sum(vec![8, 6, 1, 5, 3]), 9);
}

#[test]
fn test_minimum_sum2() {
    assert_eq!(Solution::minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
}

#[test]
fn test_minimum_sum3() {
    assert_eq!(Solution::minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
}
