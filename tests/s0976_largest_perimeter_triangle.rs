// Tests for Problem 0976: Largest Perimeter Triangle
// Java reference: src/test/java/g0901_1000/s0976_largest_perimeter_triangle/SolutionTest.java

use leetcode_in_rust::s0976::largest_perimeter_triangle::Solution;

#[test]
fn test_largest_perimeter() {
    let result = Solution::largest_perimeter(vec![2, 1, 2]);
    assert_eq!(result, 5);
}

#[test]
fn test_largest_perimeter2() {
    let result = Solution::largest_perimeter(vec![1, 2, 1]);
    assert_eq!(result, 0);
}
