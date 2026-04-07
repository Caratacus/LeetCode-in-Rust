// Tests for Problem 1942: The Number of the Smallest Unoccupied Chair
// Java reference: src/test/java/g1901_2000/s1942_the_number_of_the_smallest_unoccupied_chair/SolutionTest.java

use leetcode_in_rust::s1942::the_number_of_the_smallest_unoccupied_chair::Solution;

#[test]
fn test_smallest_chair() {
    assert_eq!(
        Solution::smallest_chair(vec![vec![1, 4], vec![2, 3], vec![4, 6]], 1),
        1
    );
}

#[test]
fn test_smallest_chair2() {
    assert_eq!(
        Solution::smallest_chair(vec![vec![3, 10], vec![1, 5], vec![2, 6]], 0),
        2
    );
}
