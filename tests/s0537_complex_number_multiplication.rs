// Tests for Problem 0537: Complex Number Multiplication
// Java reference: src/test/java/g0501_0600/s0537_complex_number_multiplication/SolutionTest.java

use leetcode_in_rust::s0537::complex_number_multiplication::Solution;

#[test]
fn test_complex_number_multiply() {
    assert_eq!(
        Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string()),
        "0+2i"
    );
}

#[test]
fn test_complex_number_multiply2() {
    assert_eq!(
        Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()),
        "0+-2i"
    );
}
