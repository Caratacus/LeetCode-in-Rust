// Tests for Problem 3448: Count Substrings Divisible by Last Digit
// Java reference: src/test/java/g3401_3500/s3448_count_substrings_divisible_by_last_digit/SolutionTest.java

use leetcode_in_rust::s3448::count_substrings_divisible_by_last_digit::Solution;

#[test]
fn test_count_substrings() {
    assert_eq!(Solution::count_substrings("12936".to_string()), 11i64);
}

#[test]
fn test_count_substrings2() {
    assert_eq!(Solution::count_substrings("5701283".to_string()), 18i64);
}

#[test]
fn test_count_substrings3() {
    assert_eq!(Solution::count_substrings("1010101010".to_string()), 25i64);
}

#[test]
fn test_count_substrings4() {
    assert_eq!(Solution::count_substrings("4".to_string()), 1i64);
}

#[test]
fn test_count_substrings5() {
    assert_eq!(Solution::count_substrings("28".to_string()), 2i64);
}

#[test]
fn test_count_substrings6() {
    assert_eq!(Solution::count_substrings("04".to_string()), 2i64);
}

#[test]
fn test_count_substrings7() {
    assert_eq!(Solution::count_substrings("8".to_string()), 1i64);
}
