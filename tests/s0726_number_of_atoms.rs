// Tests for Problem 0726: Number of Atoms
// Java reference: src/test/java/g0701_0800/s0726_number_of_atoms/SolutionTest.java

use leetcode_in_rust::s0726::number_of_atoms::Solution;

#[test]
fn test_count_of_atoms() {
    assert_eq!(Solution::count_of_atoms("H2O".to_string()), "H2O");
}

#[test]
fn test_count_of_atoms2() {
    assert_eq!(Solution::count_of_atoms("Mg(OH)2".to_string()), "H2MgO2");
}

#[test]
fn test_count_of_atoms3() {
    assert_eq!(
        Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
        "K4N2O14S4"
    );
}
