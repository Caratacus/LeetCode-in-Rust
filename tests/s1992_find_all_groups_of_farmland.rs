// Tests for Problem 1992: Find All Groups of Farmland
// Java reference: src/test/java/g1901_2000/s1992_find_all_groups_of_farmland/SolutionTest.java

use leetcode_in_rust::s1992::find_all_groups_of_farmland::Solution;

#[test]
fn test_find_farmland() {
    assert_eq!(
        Solution::find_farmland(vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]]),
        vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]]
    );
}

#[test]
fn test_find_farmland2() {
    assert_eq!(
        Solution::find_farmland(vec![vec![1, 1], vec![1, 1]]),
        vec![vec![0, 0, 1, 1]]
    );
}

#[test]
fn test_find_farmland3() {
    assert_eq!(
        Solution::find_farmland(vec![vec![0]]),
        Vec::<Vec<i32>>::new()
    );
}

#[test]
fn test_find_farmland4() {
    assert_eq!(
        Solution::find_farmland(Vec::<Vec<i32>>::new()),
        Vec::<Vec<i32>>::new()
    );
}
