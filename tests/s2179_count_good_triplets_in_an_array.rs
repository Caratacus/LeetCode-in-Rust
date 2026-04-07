// Tests for Problem 2179: Count Good Triplets in an Array
// Java reference: src/test/java/g2101_2200/s2179_count_good_triplets_in_an_array/SolutionTest.java

use leetcode_in_rust::s2179::count_good_triplets_in_an_array::Solution;

#[test]
fn test_good_triplets() {
    assert_eq!(Solution::good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
}

#[test]
fn test_good_triplets2() {
    assert_eq!(Solution::good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
}
