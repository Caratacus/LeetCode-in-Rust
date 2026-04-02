// Tests for Problem 0412: Fizz Buzz
// Java reference: src/test/java/g0401_0500/s0412_fizz_buzz/SolutionTest.java

use leetcode_in_rust::s0412::fizz_buzz::Solution;

#[test]
fn test_fizz_buzz() {
    let result = Solution::fizz_buzz(3);
    assert_eq!(result, vec!["1", "2", "Fizz"]);
}

#[test]
fn test_fizz_buzz2() {
    let result = Solution::fizz_buzz(5);
    assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);
}

#[test]
fn test_fizz_buzz3() {
    let result = Solution::fizz_buzz(15);
    assert_eq!(result[14], "FizzBuzz");
}
