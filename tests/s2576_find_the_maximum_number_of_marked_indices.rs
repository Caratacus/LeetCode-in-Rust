// Tests for Problem 2576: Find the Maximum Number of Marked Indices
// Java reference: src/test/java/g2501_2600/s2576_find_the_maximum_number_of_marked_indices/SolutionTest.java

use leetcode_in_rust::s2576::find_the_maximum_number_of_marked_indices::Solution;

#[test]
fn test_max_num_of_marked_indices() {
    assert_eq!(Solution::max_num_of_marked_indices(vec![3, 5, 2, 4]), 2);
}

#[test]
fn test_max_num_of_marked_indices2() {
    assert_eq!(Solution::max_num_of_marked_indices(vec![9, 2, 5, 4]), 4);
}

#[test]
fn test_max_num_of_marked_indices3() {
    assert_eq!(Solution::max_num_of_marked_indices(vec![7, 6, 8]), 0);
}
