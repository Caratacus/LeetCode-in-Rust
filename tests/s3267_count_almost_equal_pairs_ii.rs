// Tests for Problem 3267: Count Almost Equal Pairs II
// Java reference: src/test/java/g3201_3300/s3267_count_almost_equal_pairs_ii/SolutionTest.java

use leetcode_in_rust::s3267::count_almost_equal_pairs_ii::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![1023, 2310, 2130, 213]), 4);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![1, 10, 100]), 3);
}
