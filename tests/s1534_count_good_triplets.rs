// Tests for Problem 1534: Count Good Triplets
// Java reference: src/test/java/g1501_1600/s1534_count_good_triplets/SolutionTest.java

use leetcode_in_rust::s1534::count_good_triplets::Solution;

#[test]
fn test_count_good_triplets() {
    assert_eq!(Solution::count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
}

#[test]
fn test_count_good_triplets2() {
    assert_eq!(Solution::count_good_triplets(vec![1, 1, 2, 2, 3], 0, 0, 1), 0);
}
