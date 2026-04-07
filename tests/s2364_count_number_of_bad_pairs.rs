// Tests for Problem 2364: Count Number of Bad Pairs
// Java reference: src/test/java/g2301_2400/s2364_count_number_of_bad_pairs/SolutionTest.java

use leetcode_in_rust::s2364::count_number_of_bad_pairs::Solution;

#[test]
fn test_count_bad_pairs() {
    assert_eq!(Solution::count_bad_pairs(vec![4, 1, 3, 3]), 5);
}

#[test]
fn test_count_bad_pairs2() {
    assert_eq!(Solution::count_bad_pairs(vec![1, 2, 3, 4, 5]), 0);
}
