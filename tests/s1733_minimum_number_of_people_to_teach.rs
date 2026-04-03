// Tests for Problem 1733: Minimum Number of People to Teach
// Java reference: src/test/java/g1701_1800/s1733_minimum_number_of_people_to_teach/SolutionTest.java

use leetcode_in_rust::s1733::minimum_number_of_people_to_teach::Solution;

#[test]
fn test_minimum_teachings() {
    assert_eq!(
        Solution::minimum_teachings(
            2,
            vec![vec![1], vec![2], vec![1, 2]],
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        ),
        1
    );
}

#[test]
fn test_minimum_teachings2() {
    assert_eq!(
        Solution::minimum_teachings(
            3,
            vec![vec![2], vec![1, 3], vec![1, 2], vec![3]],
            vec![vec![1, 4], vec![1, 2], vec![1, 3], vec![3, 4], vec![2, 3]]
        ),
        2
    );
}
