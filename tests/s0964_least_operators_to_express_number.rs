// Tests for Problem 0964: Least Operators to Express Number
// Java reference: src/test/java/g0901_1000/s0964_least_operators_to_express_number/SolutionTest.java

use leetcode_in_rust::s0964::least_operators_to_express_number::Solution;

#[test]
fn test_least_ops_express_target() {
    let result = Solution::least_ops_express_target(3, 19);
    assert_eq!(result, 5);
}

#[test]
fn test_least_ops_express_target2() {
    let result = Solution::least_ops_express_target(5, 501);
    assert_eq!(result, 8);
}

#[test]
fn test_least_ops_express_target3() {
    let result = Solution::least_ops_express_target(100, 100000000);
    assert_eq!(result, 3);
}
