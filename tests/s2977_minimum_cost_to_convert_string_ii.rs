// Tests for Problem 2977: Minimum Cost to Convert String II
// Java reference: src/test/java/g2901_3000/s2977_minimum_cost_to_convert_string_ii/SolutionTest.java

use leetcode_in_rust::s2977::minimum_cost_to_convert_string_ii::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string(), "c".to_string(), "e".to_string(), "d".to_string()],
            vec!["b".to_string(), "c".to_string(), "b".to_string(), "e".to_string(), "b".to_string(), "e".to_string()],
            vec![2, 5, 5, 1, 2, 20]
        ),
        28
    );
}
