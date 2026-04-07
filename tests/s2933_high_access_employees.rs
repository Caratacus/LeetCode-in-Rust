// Tests for Problem 2933: High Access Employees
// Java reference: src/test/java/g2901_3000/s2933_high_access_employees/SolutionTest.java

use leetcode_in_rust::s2933::high_access_employees::Solution;

#[test]
fn test_find_high_access_employees() {
    let result = Solution::find_high_access_employees(vec![
        vec!["a".to_string(), "0549".to_string()],
        vec!["b".to_string(), "0457".to_string()],
        vec!["a".to_string(), "0532".to_string()],
        vec!["a".to_string(), "0621".to_string()],
        vec!["b".to_string(), "0540".to_string()],
    ]);
    assert!(result.contains(&"a".to_string()));
}
