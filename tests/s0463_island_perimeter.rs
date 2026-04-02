// Tests for Problem 0463: Island Perimeter
// Java reference: src/test/java/g0401_0500/s0463_island_perimeter/SolutionTest.java

use leetcode_in_rust::s0463::island_perimeter::Solution;

#[test]
fn test_island_perimeter() {
    assert_eq!(
        Solution::island_perimeter(vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0]
        ]),
        16
    );
}

#[test]
fn test_island_perimeter2() {
    assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
}

#[test]
fn test_island_perimeter3() {
    assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
}
