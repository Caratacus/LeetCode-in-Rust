// Tests for Problem 1705: Maximum Number of Eaten Apples
// Java reference: src/test/java/g1701_1800/s1705_maximum_number_of_eaten_apples/SolutionTest.java

use leetcode_in_rust::s1705::maximum_number_of_eaten_apples::Solution;

#[test]
fn test_eaten_apples() {
    assert_eq!(
        Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2]),
        7
    );
}

#[test]
fn test_eaten_apples2() {
    assert_eq!(
        Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2]),
        5
    );
}
