// Tests for Problem 2215: Find the Difference of Two Arrays
// Java reference: src/test/java/g2201_2300/s2215_find_the_difference_of_two_arrays/SolutionTest.java

use leetcode_in_rust::s2215::find_the_difference_of_two_arrays::Solution;

#[test]
fn test_find_difference() {
    let result = Solution::find_difference(vec![1, 2, 3], vec![2, 4, 6]);
    // Sort for comparison since order doesn't matter
    let mut list1 = result[0].clone();
    let mut list2 = result[1].clone();
    list1.sort();
    list2.sort();
    assert_eq!(list1, vec![1, 3]);
    assert_eq!(list2, vec![4, 6]);
}

#[test]
fn test_find_difference2() {
    let result = Solution::find_difference(vec![1, 2, 3, 3], vec![1, 1, 2, 2]);
    // Sort for comparison since order doesn't matter
    let mut list1 = result[0].clone();
    let mut list2 = result[1].clone();
    list1.sort();
    list2.sort();
    assert_eq!(list1, vec![3]);
    assert_eq!(list2, vec![]);
}
