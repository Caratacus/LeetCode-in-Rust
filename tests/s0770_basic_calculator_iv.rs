// Tests for Problem 0770: Basic Calculator IV
// Java reference: src/test/java/g0701_0800/s0770_basic_calculator_iv/SolutionTest.java

use leetcode_in_rust::s0770::basic_calculator_iv::Solution;

#[test]
fn test_basic_calculator_iv() {
    assert_eq!(
        Solution::basic_calculator_iv(
            "e + 8 - a + 5".to_string(),
            vec!["e".to_string()],
            vec![1]
        ),
        vec!["-1*a".to_string(), "14".to_string()]
    );
}

#[test]
fn test_basic_calculator_iv2() {
    assert_eq!(
        Solution::basic_calculator_iv(
            "e - 8 + temperature - pressure".to_string(),
            vec!["e".to_string(), "temperature".to_string()],
            vec![1, 12]
        ),
        vec!["-1*pressure".to_string(), "5".to_string()]
    );
}

#[test]
fn test_basic_calculator_iv3() {
    assert_eq!(
        Solution::basic_calculator_iv(
            "(e + 8) * (e - 8)".to_string(),
            vec![],
            vec![]
        ),
        vec!["1*e*e".to_string(), "-64".to_string()]
    );
}
