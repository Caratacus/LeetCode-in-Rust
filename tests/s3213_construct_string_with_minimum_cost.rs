// Tests for Problem 3213: Construct String With Minimum Cost
// Java reference: src/test/java/g3201_3300/s3213_construct_string_with_minimum_cost/SolutionTest.java

use leetcode_in_rust::s3213::construct_string_with_minimum_cost::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(
        Solution::minimum_cost(
            "abcdef".to_string(),
            vec!["abdef".to_string(), "abc".to_string(), "d".to_string(), "def".to_string(), "ef".to_string()],
            vec![100, 1, 1, 10, 5]
        ),
        7
    );
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(
        Solution::minimum_cost(
            "aaaa".to_string(),
            vec!["z".to_string(), "zz".to_string(), "zzz".to_string()],
            vec![1, 10, 100]
        ),
        -1
    );
}
