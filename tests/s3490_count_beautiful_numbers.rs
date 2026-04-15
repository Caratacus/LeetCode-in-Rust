// Tests for Problem 3490: Count Beautiful Numbers
// Java reference: src/test/java/g3401_3500/s3490_count_beautiful_numbers/SolutionTest.java

use leetcode_in_rust::s3490::count_beautiful_numbers::Solution;

#[test]
fn test_beautiful_numbers() {
    assert_eq!(Solution::beautiful_numbers(10, 20), 2);
}

#[test]
fn test_beautiful_numbers2() {
    assert_eq!(Solution::beautiful_numbers(1, 15), 10);
}

#[test]
fn test_beautiful_numbers3() {
    assert_eq!(Solution::beautiful_numbers(6725, 270910825), 178996547);
}
