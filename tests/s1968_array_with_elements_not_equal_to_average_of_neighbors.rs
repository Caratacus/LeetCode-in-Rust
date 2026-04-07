// Tests for Problem 1968: Array With Elements Not Equal to Average of Neighbors
// Java reference: src/test/java/g1901_2000/s1968_array_with_elements_not_equal_to_average_of_neighbors/SolutionTest.java

use leetcode_in_rust::s1968::array_with_elements_not_equal_to_average_of_neighbors::Solution;

fn is_valid_rearrangement(result: &[i32]) -> bool {
    if result.len() < 3 {
        return true;
    }
    for i in 1..result.len() - 1 {
        let avg = (result[i - 1] + result[i + 1]) as f64 / 2.0;
        if (result[i] as f64 - avg).abs() < f64::EPSILON {
            return false;
        }
    }
    true
}

#[test]
fn test_rearrange_array() {
    let result = Solution::rearrange_array(vec![1, 2, 3, 4, 5]);
    assert!(is_valid_rearrangement(&result));
    assert_eq!(result.len(), 5);
}

#[test]
fn test_rearrange_array2() {
    let result = Solution::rearrange_array(vec![6, 2, 0, 9, 7]);
    assert!(is_valid_rearrangement(&result));
    assert_eq!(result.len(), 5);
}
