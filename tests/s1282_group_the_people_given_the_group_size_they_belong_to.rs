// Tests for Problem 1282: Group the People Given the Group Size They Belong To
// Java reference: src/test/java/g1201_1300/s1282_group_the_people_given_the_group_size_they_belong_to/SolutionTest.java

use leetcode_in_rust::s1282::group_the_people_given_the_group_size_they_belong_to::Solution;

#[test]
fn test_group_the_people() {
    let mut result = Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]);
    for group in &mut result {
        group.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]);
}

#[test]
fn test_group_the_people2() {
    let mut result = Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]);
    for group in &mut result {
        group.sort();
    }
    result.sort();
    assert_eq!(result, vec![vec![1], vec![0, 5], vec![2, 3, 4]]);
}
