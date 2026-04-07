// Tests for Problem 2961: Double Modular Exponentiation
// Java reference: src/test/java/g2901_3000/s2961_double_modular_exponentiation/SolutionTest.java

use leetcode_in_rust::s2961::double_modular_exponentiation::Solution;

#[test]
fn test_get_good_indices() {
    assert_eq!(
        Solution::get_good_indices(vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]], 2),
        vec![0, 2]
    );
}

#[test]
fn test_get_good_indices2() {
    assert_eq!(
        Solution::get_good_indices(vec![vec![39, 3, 1000, 1000]], 17),
        vec![] as Vec<i32>
    );
}
