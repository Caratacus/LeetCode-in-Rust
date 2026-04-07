// Tests for Problem 1847: Closest Room
// Java reference: src/test/java/g1801_1900/s1847_closest_room/SolutionTest.java

use leetcode_in_rust::s1847::closest_room::Solution;

#[test]
fn test_closest_room() {
    assert_eq!(
        Solution::closest_room(
            vec![vec![2, 2], vec![1, 2], vec![3, 2]],
            vec![vec![3, 1], vec![3, 3], vec![5, 2]]
        ),
        vec![2, -1, 2]
    );
}

#[test]
fn test_closest_room2() {
    assert_eq!(
        Solution::closest_room(
            vec![vec![1, 4], vec![2, 3], vec![3, 5], vec![4, 1], vec![5, 2]],
            vec![vec![2, 3], vec![2, 4], vec![2, 5]]
        ),
        vec![2, 1, 3]
    );
}
