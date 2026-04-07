// Tests for Problem 2572: Count the Number of Square-Free Subsets
// Java reference: src/test/java/g2501_2600/s2572_count_the_number_of_square_free_subsets/SolutionTest.java

use leetcode_in_rust::s2572::count_the_number_of_square_free_subsets::Solution;

#[test]
fn test_square_free_subsets() {
    assert_eq!(Solution::square_free_subsets(vec![3, 4, 4, 5]), 3);
}

#[test]
fn test_square_free_subsets2() {
    assert_eq!(Solution::square_free_subsets(vec![1]), 1);
}
