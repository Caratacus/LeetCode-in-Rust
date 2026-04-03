// Tests for Problem 1157: Online Majority Element in Subarray
// Java reference: src/test/java/g1101_1200/s1157_online_majority_element_in_subarray/SolutionTest.java

use leetcode_in_rust::s1157::online_majority_element_in_subarray::MajorityChecker;

#[test]
fn test_majority_checker_test() {
    let mut majority_checker = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(majority_checker.query(0, 5, 4), 1);
    assert_eq!(majority_checker.query(0, 3, 3), -1);
    assert_eq!(majority_checker.query(2, 3, 2), 2);
}
