// Tests for Problem 3723: Maximize Sum of Squares of Digits
// Java reference: src/test/java/g3701_3800/s3723_maximize_sum_of_squares_of_digits/SolutionTest.java
use leetcode_in_rust::s3723::maximize_sum_of_squares_of_digits::Solution;
#[test]
fn test_max_sum_of_squares() { assert_eq!(Solution::max_sum_of_squares(2, 3), "30".to_string()); }
#[test]
fn test_max_sum_of_squares2() { assert_eq!(Solution::max_sum_of_squares(2, 17), "98".to_string()); }
#[test]
fn test_max_sum_of_squares3() { assert_eq!(Solution::max_sum_of_squares(1, 10), "".to_string()); }
#[test]
fn test_max_sum_of_squares4() { assert_eq!(Solution::max_sum_of_squares(2, 27), "".to_string()); }
#[test]
fn test_max_sum_of_squares5() { assert_eq!(Solution::max_sum_of_squares(3, 28), "".to_string()); }
#[test]
fn test_max_sum_of_squares6() { assert_eq!(Solution::max_sum_of_squares(3, 27), "999".to_string()); }
#[test]
fn test_max_sum_of_squares7() { assert_eq!(Solution::max_sum_of_squares(2, 10), "91".to_string()); }
#[test]
fn test_max_sum_of_squares8() { assert_eq!(Solution::max_sum_of_squares(4, 10), "9100".to_string()); }
#[test]
fn test_max_sum_of_squares9() { assert_eq!(Solution::max_sum_of_squares(3, 5), "500".to_string()); }
