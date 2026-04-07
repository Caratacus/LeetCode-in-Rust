// Tests for Problem 2248: Intersection of Multiple Arrays
// Java reference: src/test/java/g2201_2300/s2248_intersection_of_multiple_arrays/SolutionTest.java

use leetcode_in_rust::s2248::intersection_of_multiple_arrays::Solution;

#[test]
fn test_intersection() {
    let result = Solution::intersection(vec![
        vec![3, 1, 2, 4, 5],
        vec![1, 2, 3, 4],
        vec![3, 4, 5, 6],
    ]);
    assert_eq!(result, vec![3, 4]);
}

#[test]
fn test_intersection2() {
    let result = Solution::intersection(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ]);
    assert_eq!(result, Vec::<i32>::new());
}
