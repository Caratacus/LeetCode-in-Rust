// Tests for Problem 0399: Evaluate Division
// Java reference: src/test/java/g0301_0400/s0399_evaluate_division/SolutionTest.java

use leetcode_in_rust::s0399::evaluate_division::Solution;

#[test]
fn test_calc_equation() {
    let equations = vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()]];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    let expected = vec![6.0, 0.5, -1.0, 1.0, -1.0];
    let result = Solution::calc_equation(equations, values, queries);
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!((r - e).abs() < 1e-5, "Query {} failed: expected {}, got {}", i, e, r);
    }
}

#[test]
fn test_calc_equation2() {
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
    ];
    let values = vec![1.5, 2.5, 5.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["c".to_string(), "b".to_string()],
        vec!["bc".to_string(), "cd".to_string()],
        vec!["cd".to_string(), "bc".to_string()],
    ];
    let expected = vec![3.75, 0.4, 5.0, 0.2];
    let result = Solution::calc_equation(equations, values, queries);
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!((r - e).abs() < 1e-5, "Query {} failed: expected {}, got {}", i, e, r);
    }
}

#[test]
fn test_calc_equation3() {
    let equations = vec![vec!["a".to_string(), "b".to_string()]];
    let values = vec![0.5];
    let queries = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "c".to_string()],
        vec!["x".to_string(), "y".to_string()],
    ];
    let expected = vec![0.5, 2.0, -1.0, -1.0];
    let result = Solution::calc_equation(equations, values, queries);
    for (i, (r, e)) in result.iter().zip(expected.iter()).enumerate() {
        assert!((r - e).abs() < 1e-5, "Query {} failed: expected {}, got {}", i, e, r);
    }
}
