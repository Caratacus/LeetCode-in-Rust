// Tests for Problem 2375: Construct Smallest Number From DI String
// Java reference: src/test/java/g2301_2400/s2375_construct_smallest_number_from_di_string/SolutionTest.java

use leetcode_in_rust::s2375::construct_smallest_number_from_di_string::Solution;

#[test]
fn test_smallest_number() {
    assert_eq!(Solution::smallest_number("IIIDIDDD".to_string()), "123549876");
}

#[test]
fn test_smallest_number2() {
    assert_eq!(Solution::smallest_number("DDD".to_string()), "4321");
}
