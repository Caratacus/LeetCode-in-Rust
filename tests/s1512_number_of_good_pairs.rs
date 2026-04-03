// Tests for Problem 1512: Number of Good Pairs
// Java reference: src/test/java/g1501_1600/s1512_number_of_good_pairs/SolutionTest.java

use leetcode_in_rust::s1512::number_of_good_pairs::Solution;

#[test]
fn test_num_identical_pairs() {
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
}

#[test]
fn test_num_identical_pairs2() {
    assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
}

#[test]
fn test_num_identical_pairs3() {
    assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
}
