// Tests for Problem 1442: Count Triplets That Can Form Two Arrays of Equal XOR
// Java reference: src/test/java/g1401_1500/s1442_count_triplets_that_can_form_two_arrays_of_equal_xor/SolutionTest.java

use leetcode_in_rust::s1442::count_triplets_that_can_form_two_arrays_of_equal_xor::Solution;

#[test]
fn test_count_triplets() {
    assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
}

#[test]
fn test_count_triplets2() {
    assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
}
