// Tests for Problem 1985: Find the Kth Largest Integer in the Array
// Java reference: src/test/java/g1901_2000/s1985_find_the_kth_largest_integer_in_the_array/SolutionTest.java

use leetcode_in_rust::s1985::find_the_kth_largest_integer_in_the_array::Solution;

#[test]
fn test_kth_largest_number() {
    assert_eq!(
        Solution::kth_largest_number(vec!["3".to_string(), "6".to_string(), "7".to_string(), "10".to_string()], 4),
        "3"
    );
}

#[test]
fn test_kth_largest_number2() {
    assert_eq!(
        Solution::kth_largest_number(vec!["2".to_string(), "21".to_string(), "12".to_string(), "1".to_string()], 3),
        "2"
    );
}

#[test]
fn test_kth_largest_number3() {
    assert_eq!(
        Solution::kth_largest_number(vec!["0".to_string(), "0".to_string()], 2),
        "0"
    );
}
