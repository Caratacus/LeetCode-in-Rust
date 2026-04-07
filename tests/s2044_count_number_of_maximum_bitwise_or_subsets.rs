// Tests for Problem 2044: Count Number of Maximum Bitwise-Or Subsets
// Java reference: src/test/java/g2001_2100/s2044_count_number_of_maximum_bitwise_or_subsets/SolutionTest.java

use leetcode_in_rust::s2044::count_number_of_maximum_bitwise_or_subsets::Solution;

#[test]
fn test_count_max_or_subsets() {
    assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
}

#[test]
fn test_count_max_or_subsets2() {
    assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
}

#[test]
fn test_count_max_or_subsets3() {
    assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
}
