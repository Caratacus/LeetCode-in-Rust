// Tests for Problem 0749: Contain Virus
// Java reference: src/test/java/g0701_0800/s0749_contain_virus/SolutionTest.java

use leetcode_in_rust::s0749::contain_virus::Solution;

#[test]
fn test_contain_virus() {
    let input = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(input), 10);
}

#[test]
fn test_contain_virus2() {
    let input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    assert_eq!(Solution::contain_virus(input), 4);
}

#[test]
fn test_contain_virus3() {
    let input = vec![
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(input), 13);
}
