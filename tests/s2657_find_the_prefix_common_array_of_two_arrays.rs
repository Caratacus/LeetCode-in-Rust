// Tests for Problem 2657: Find the Prefix Common Array of Two Arrays
// Java reference: src/test/java/g2601_2700/s2657_find_the_prefix_common_array_of_two_arrays/SolutionTest.java

use leetcode_in_rust::s2657::find_the_prefix_common_array_of_two_arrays::Solution;

#[test]
fn test_find_the_prefix_common_array() {
    assert_eq!(
        Solution::find_the_prefix_common_array(vec![1, 3, 2, 4], vec![3, 1, 2, 4]),
        vec![0, 2, 3, 4]
    );
}

#[test]
fn test_find_the_prefix_common_array2() {
    assert_eq!(
        Solution::find_the_prefix_common_array(vec![2, 3, 1], vec![3, 1, 2]),
        vec![0, 1, 3]
    );
}
