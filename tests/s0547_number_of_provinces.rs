// Tests for Problem 0547: Number of Provinces
// Java reference: src/test/java/g0501_0600/s0547_number_of_provinces/SolutionTest.java

use leetcode_in_rust::s0547::number_of_provinces::Solution;

#[test]
fn test_find_circle_num() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    );
}

#[test]
fn test_find_circle_num2() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}
