// Tests for Problem 2092: Find All People With Secret
// Java reference: src/test/java/g2001_2100/s2092_find_all_people_with_secret/SolutionTest.java

use leetcode_in_rust::s2092::find_all_people_with_secret::Solution;

#[test]
fn test_find_all_people() {
    let mut result = Solution::find_all_people(
        6,
        vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]],
        1,
    );
    result.sort();
    assert_eq!(result, vec![0, 1, 2, 3, 5]);
}

#[test]
fn test_find_all_people2() {
    let mut result = Solution::find_all_people(
        4,
        vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]],
        3,
    );
    result.sort();
    assert_eq!(result, vec![0, 1, 3]);
}

#[test]
fn test_find_all_people3() {
    let mut result = Solution::find_all_people(
        5,
        vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]],
        1,
    );
    result.sort();
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}
