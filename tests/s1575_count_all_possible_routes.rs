// Tests for Problem 1575: Count All Possible Routes
// Java reference: src/test/java/g1501_1600/s1575_count_all_possible_routes/SolutionTest.java

use leetcode_in_rust::s1575::count_all_possible_routes::Solution;

#[test]
fn test_count_routes() {
    assert_eq!(Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5), 4);
}

#[test]
fn test_count_routes2() {
    assert_eq!(Solution::count_routes(vec![4, 3, 1], 1, 0, 6), 5);
}

#[test]
fn test_count_routes3() {
    assert_eq!(Solution::count_routes(vec![5, 2, 1], 0, 2, 3), 0);
}
