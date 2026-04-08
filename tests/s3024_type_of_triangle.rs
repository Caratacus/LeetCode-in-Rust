// Tests for Problem 3024: Type of Triangle
// Java reference: src/test/java/g3001_3100/s3024_type_of_triangle/SolutionTest.java
use leetcode_in_rust::s3024::type_of_triangle::Solution;
#[test]
fn test_triangle_type() {
    assert_eq!(Solution::triangle_type(vec![3, 3, 3]), String::from("equilateral"));
}
#[test]
fn test_triangle_type2() {
    assert_eq!(Solution::triangle_type(vec![3, 4, 5]), String::from("scalene"));
}
#[test]
fn test_triangle_type3() {
    assert_eq!(Solution::triangle_type(vec![5, 5, 3]), String::from("isosceles"));
}
#[test]
fn test_triangle_type4() {
    assert_eq!(Solution::triangle_type(vec![1, 2, 3]), String::from("none"));
}
#[test]
fn test_triangle_type5() {
    assert_eq!(Solution::triangle_type(vec![100, 100, 100]), String::from("equilateral"));
}
#[test]
fn test_triangle_type6() {
    assert_eq!(Solution::triangle_type(vec![7, 10, 7]), String::from("isosceles"));
}
#[test]
fn test_triangle_type7() {
    assert_eq!(Solution::triangle_type(vec![0, 4, 4]), String::from("none"));
}
#[test]
fn test_triangle_type8() {
    assert_eq!(Solution::triangle_type(vec![-3, 4, 5]), String::from("none"));
}
#[test]
fn test_triangle_type9() {
    // 2 + 3 = 5 -> equals, not greater -> invalid
    assert_eq!(Solution::triangle_type(vec![2, 3, 5]), String::from("none"));
}
#[test]
fn test_triangle_type10() {
    assert_eq!(Solution::triangle_type(vec![5, 3, 4]), String::from("scalene"));
}
#[test]
fn test_triangle_type11() {
    assert_eq!(Solution::triangle_type(vec![4, 6, 6]), String::from("isosceles"));
}
#[test]
fn test_triangle_type12() {
    assert_eq!(Solution::triangle_type(vec![0, 0, 0]), String::from("none"));
}
#[test]
fn test_triangle_type13() {
    assert_eq!(Solution::triangle_type(vec![5, 5, 6]), String::from("isosceles"));
}
#[test]
fn test_triangle_type14() {
    assert_eq!(Solution::triangle_type(vec![10, 11, 12]), String::from("scalene"));
}
#[test]
fn test_triangle_type15() {
    assert_eq!(Solution::triangle_type(vec![1, 10, 12]), String::from("none"));
}
