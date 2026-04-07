// Tests for Problem 2861: Maximum Number of Alloys
// Java reference: src/test/java/g2801_2900/s2861_maximum_number_of_alloys/SolutionTest.java

use leetcode_in_rust::s2861::maximum_number_of_alloys::Solution;

#[test]
fn test_max_number_of_alloys() {
    assert_eq!(
        Solution::max_number_of_alloys(
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 0],
            vec![1, 2, 3]
        ),
        2
    );
}

#[test]
fn test_max_number_of_alloys2() {
    assert_eq!(
        Solution::max_number_of_alloys(
            3,
            2,
            15,
            vec![vec![1, 1, 1], vec![1, 1, 10]],
            vec![0, 0, 100],
            vec![1, 2, 3]
        ),
        5
    );
}

#[test]
fn test_max_number_of_alloys3() {
    assert_eq!(
        Solution::max_number_of_alloys(
            2,
            3,
            10,
            vec![vec![2, 1], vec![1, 2], vec![1, 1]],
            vec![1, 1],
            vec![5, 5]
        ),
        2
    );
}
