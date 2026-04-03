// Tests for Problem 0483: Smallest Good Base
// Java reference: src/test/java/g0401_0500/s0483_smallest_good_base/SolutionTest.java

use leetcode_in_rust::s0483::smallest_good_base::Solution;

#[test]
fn test_smallest_good_base() {
    assert_eq!(Solution::smallest_good_base("13".to_string()), "3");
}

#[test]
fn test_smallest_good_base2() {
    assert_eq!(Solution::smallest_good_base("4681".to_string()), "8");
}

#[test]
fn test_smallest_good_base3() {
    assert_eq!(
        Solution::smallest_good_base("1000000000000000000".to_string()),
        "999999999999999999"
    );
}
