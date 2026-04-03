// Tests for Problem 1514: Path with Maximum Probability
// Java reference: src/test/java/g1501_1600/s1514_path_with_maximum_probability/SolutionTest.java

use leetcode_in_rust::s1514::path_with_maximum_probability::Solution;

#[test]
fn test_max_probability() {
    let result = Solution::max_probability(
        3,
        vec![vec![0, 1], vec![1, 2], vec![0, 2]],
        vec![0.5, 0.5, 0.2],
        0,
        2,
    );
    assert!((result - 0.2500).abs() < 1e-5);
}

#[test]
fn test_max_probability2() {
    let result = Solution::max_probability(
        3,
        vec![vec![0, 1], vec![1, 2], vec![0, 2]],
        vec![0.5, 0.5, 0.3],
        0,
        2,
    );
    assert!((result - 0.3).abs() < 1e-5);
}

#[test]
fn test_max_probability3() {
    let result = Solution::max_probability(
        3,
        vec![vec![0, 1]],
        vec![0.5],
        0,
        2,
    );
    assert!((result - 0.0).abs() < 1e-5);
}
