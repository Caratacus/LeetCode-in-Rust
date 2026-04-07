// Tests for Problem 2399: Check Distances Between Same Letters
// Java reference: src/test/java/g2301_2400/s2399_check_distances_between_same_letters/SolutionTest.java

use leetcode_in_rust::s2399::check_distances_between_same_letters::Solution;

#[test]
fn test_check_distances() {
    let distance = vec![
        1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(Solution::check_distances("abaccb".to_string(), distance), true);
}

#[test]
fn test_check_distances2() {
    let distance = vec![
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(Solution::check_distances("aa".to_string(), distance), false);
}
