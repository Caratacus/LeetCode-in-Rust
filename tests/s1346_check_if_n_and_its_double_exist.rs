// Tests for Problem 1346: Check If N and Its Double Exist
// Java reference: src/test/java/g1301_1400/s1346_check_if_n_and_its_double_exist/SolutionTest.java

use leetcode_in_rust::s1346::check_if_n_and_its_double_exist::Solution;

#[test]
fn test_check_if_exist() {
    assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
}

#[test]
fn test_check_if_exist2() {
    assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
}

#[test]
fn test_check_if_exist3() {
    assert_eq!(Solution::check_if_exist(vec![-2, 0, 10, -19, 4, 6, -8]), false);
}

#[test]
fn test_check_if_exist4() {
    assert_eq!(Solution::check_if_exist(vec![0, 0]), true);
}
