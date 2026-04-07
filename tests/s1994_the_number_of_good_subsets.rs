// Tests for Problem 1994: The Number of Good Subsets
// Java reference: src/test/java/g1901_2000/s1994_the_number_of_good_subsets/SolutionTest.java

use leetcode_in_rust::s1994::the_number_of_good_subsets::Solution;

#[test]
fn test_number_of_good_subsets() {
    assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 3, 4]), 6);
}

#[test]
fn test_number_of_good_subsets2() {
    assert_eq!(Solution::number_of_good_subsets(vec![4, 2, 3, 15]), 5);
}
