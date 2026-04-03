// Tests for Problem 1299: Replace Elements with Greatest Element on Right Side
// Java reference: src/test/java/g1201_1300/s1299_replace_elements_with_greatest_element_on_right_side/SolutionTest.java

use leetcode_in_rust::s1299::replace_elements_with_greatest_element_on_right_side::Solution;

#[test]
fn test_replace_elements() {
    assert_eq!(
        Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
}

#[test]
fn test_replace_elements2() {
    assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
}
