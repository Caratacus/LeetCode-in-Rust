// Tests for Problem 2976: Minimum Cost to Convert String I
// Java reference: src/test/java/g2901_3000/s2976_minimum_cost_to_convert_string_i/SolutionTest.java

use leetcode_in_rust::s2976::minimum_cost_to_convert_string_i::Solution;

#[test]
fn test_minimum_cost() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "acbe".to_string(),
            vec!['a', 'b', 'c', 'c', 'e', 'd'],
            vec!['b', 'c', 'b', 'e', 'b', 'e'],
            vec![2, 5, 5, 1, 2, 20]
        ),
        28
    );
}

#[test]
fn test_minimum_cost2() {
    assert_eq!(
        Solution::minimum_cost(
            "aaaa".to_string(),
            "bbbb".to_string(),
            vec!['a', 'c'],
            vec!['c', 'b'],
            vec![1, 2]
        ),
        12
    );
}

#[test]
fn test_minimum_cost3() {
    assert_eq!(
        Solution::minimum_cost(
            "abcd".to_string(),
            "abce".to_string(),
            vec!['a'],
            vec!['e'],
            vec![1000]
        ),
        -1
    );
}
