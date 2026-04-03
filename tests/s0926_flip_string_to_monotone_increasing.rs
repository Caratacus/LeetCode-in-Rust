// Tests for Problem 0926: Flip String to Monotone Increasing
// Java reference: src/test/java/g0901_1000/s0926_flip_string_to_monotone_increasing/SolutionTest.java

use leetcode_in_rust::s0926::flip_string_to_monotone_increasing::Solution;

#[test]
fn test_min_flips_mono_incr() {
    let result = Solution::min_flips_mono_incr("00110".to_string());
    assert_eq!(result, 1);
}

#[test]
fn test_min_flips_mono_incr2() {
    let result = Solution::min_flips_mono_incr("010110".to_string());
    assert_eq!(result, 2);
}

#[test]
fn test_min_flips_mono_incr3() {
    let result = Solution::min_flips_mono_incr("00011000".to_string());
    assert_eq!(result, 2);
}
