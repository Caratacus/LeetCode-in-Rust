// Tests for Problem 0038: Count and Say
// Java reference: src/test/java/g0001_0100/s0038_count_and_say/SolutionTest.java

use leetcode_in_rust::s0038::count_and_say::Solution;

#[test]
fn test_count_and_say() {
    assert_eq!(Solution::count_and_say(1), "1");
}

#[test]
fn test_count_and_say2() {
    assert_eq!(Solution::count_and_say(2), "11");
}

#[test]
fn test_count_and_say3() {
    assert_eq!(Solution::count_and_say(3), "21");
}

#[test]
fn test_count_and_say4() {
    assert_eq!(Solution::count_and_say(4), "1211");
}
