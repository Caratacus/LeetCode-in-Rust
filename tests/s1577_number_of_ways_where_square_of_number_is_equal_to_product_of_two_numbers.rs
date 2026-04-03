// Tests for Problem 1577: Number of Ways Where Square of Number Is Equal to Product of Two Numbers
// Java reference: src/test/java/g1501_1600/s1577_number_of_ways_where_square_of_number_is_equal_to_product_of_two_numbers/SolutionTest.java

use leetcode_in_rust::s1577::number_of_ways_where_square_of_number_is_equal_to_product_of_two_numbers::Solution;

#[test]
fn test_num_triplets() {
    assert_eq!(Solution::num_triplets(vec![7, 4], vec![5, 2, 8, 9]), 1);
}

#[test]
fn test_num_triplets2() {
    assert_eq!(Solution::num_triplets(vec![1, 1], vec![1, 1, 1]), 9);
}

#[test]
fn test_num_triplets3() {
    assert_eq!(Solution::num_triplets(vec![7, 7, 8, 3], vec![1, 2, 9, 7]), 2);
}
