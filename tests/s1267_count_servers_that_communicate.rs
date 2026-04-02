// Tests for Problem 1267: Count Servers that Communicate
// Java reference: src/test/java/g1201_1300/s1267_count_servers_that_communicate/SolutionTest.java

use leetcode_in_rust::s1267::count_servers_that_communicate::Solution;

#[test]
fn test_count_servers() {
    assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
}

#[test]
fn test_count_servers2() {
    assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
}

#[test]
fn test_count_servers3() {
    assert_eq!(
        Solution::count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]),
        4
    );
}
