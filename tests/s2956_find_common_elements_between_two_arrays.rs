// Tests for Problem 2956: Find Common Elements Between Two Arrays
// Java reference: src/test/java/g2901_3000/s2956_find_common_elements_between_two_arrays/SolutionTest.java

use leetcode_in_rust::s2956::find_common_elements_between_two_arrays::Solution;

#[test]
fn test_find_intersection_values() {
    assert_eq!(
        Solution::find_intersection_values(vec![4, 3, 2, 3, 1], vec![2, 2, 5, 2, 3, 6]),
        vec![3, 4]
    );
}

#[test]
fn test_find_intersection_values2() {
    assert_eq!(
        Solution::find_intersection_values(vec![3, 4, 2, 3], vec![1, 5]),
        vec![0, 0]
    );
}
