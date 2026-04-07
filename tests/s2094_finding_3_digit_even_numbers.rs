// Tests for Problem 2094: Finding 3-Digit Even Numbers
// Java reference: src/test/java/g2001_2100/s2094_finding_3_digit_even_numbers/SolutionTest.java

use leetcode_in_rust::s2094::finding_3_digit_even_numbers::Solution;

#[test]
fn test_find_even_numbers() {
    assert_eq!(
        Solution::find_even_numbers(vec![2, 1, 3, 0]),
        vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
    );
}

#[test]
fn test_find_even_numbers2() {
    assert_eq!(
        Solution::find_even_numbers(vec![2, 2, 8, 8, 2]),
        vec![222, 228, 282, 288, 822, 828, 882]
    );
}

#[test]
fn test_find_even_numbers3() {
    assert_eq!(Solution::find_even_numbers(vec![3, 7, 5]), vec![]);
}
