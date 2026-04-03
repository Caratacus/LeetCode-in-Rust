// Tests for Problem 1388: Pizza With 3n Slices
// Java reference: src/test/java/g1301_1400/s1388_pizza_with_3n_slices/SolutionTest.java

use leetcode_in_rust::s1388::pizza_with_3n_slices::Solution;

#[test]
fn test_max_size_slices() {
    assert_eq!(Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]), 10);
}

#[test]
fn test_max_size_slices2() {
    assert_eq!(Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]), 16);
}
