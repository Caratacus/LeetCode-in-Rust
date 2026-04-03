// Tests for Problem 1574: Shortest Subarray to be Removed to Make Array Sorted
// Java reference: src/test/java/g1501_1600/s1574_shortest_subarray_to_be_removed_to_make_array_sorted/SolutionTest.java

use leetcode_in_rust::s1574::shortest_subarray_to_be_removed_to_make_array_sorted::Solution;

#[test]
fn test_find_length_of_shortest_subarray() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
        3
    );
}

#[test]
fn test_find_length_of_shortest_subarray2() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
        4
    );
}

#[test]
fn test_find_length_of_shortest_subarray3() {
    assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
}
