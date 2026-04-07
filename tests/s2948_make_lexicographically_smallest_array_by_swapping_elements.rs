// Tests for Problem 2948: Make Lexicographically Smallest Array by Swapping Elements
// Java reference: src/test/java/g2901_3000/s2948_make_lexicographically_smallest_array_by_swapping_elements/SolutionTest.java

use leetcode_in_rust::s2948::make_lexicographically_smallest_array_by_swapping_elements::Solution;

#[test]
fn test_lexicographically_smallest_array() {
    assert_eq!(
        Solution::lexicographically_smallest_array(vec![1, 5, 3, 9, 8], 2),
        vec![1, 3, 5, 8, 9]
    );
}

#[test]
fn test_lexicographically_smallest_array2() {
    assert_eq!(
        Solution::lexicographically_smallest_array(vec![1, 7, 6, 18, 2, 1], 3),
        vec![1, 6, 7, 18, 1, 2]
    );
}

#[test]
fn test_lexicographically_smallest_array3() {
    assert_eq!(
        Solution::lexicographically_smallest_array(vec![1, 7, 28, 19, 10], 3),
        vec![1, 7, 28, 19, 10]
    );
}
