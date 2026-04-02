// Tests for Problem 1185: Day of the Week
// Java reference: src/test/java/g1101_1200/s1185_day_of_the_week/SolutionTest.java

use leetcode_in_rust::s1185::day_of_the_week::Solution;

#[test]
fn test_day_of_the_week() {
    assert_eq!(Solution::day_of_the_week(31, 8, 2019), "Saturday");
}

#[test]
fn test_day_of_the_week2() {
    assert_eq!(Solution::day_of_the_week(18, 7, 1999), "Sunday");
}

#[test]
fn test_day_of_the_week3() {
    assert_eq!(Solution::day_of_the_week(15, 8, 1993), "Sunday");
}

#[test]
fn test_day_of_the_week4() {
    assert_eq!(Solution::day_of_the_week(1, 1, 1971), "Friday");
}

#[test]
fn test_day_of_the_week5() {
    assert_eq!(Solution::day_of_the_week(29, 2, 2020), "Saturday");
}

#[test]
fn test_day_of_the_week6() {
    assert_eq!(Solution::day_of_the_week(1, 3, 2020), "Sunday");
}

#[test]
fn test_day_of_the_week7() {
    assert_eq!(Solution::day_of_the_week(28, 2, 2019), "Thursday");
}

#[test]
fn test_day_of_the_week8() {
    assert_eq!(Solution::day_of_the_week(31, 12, 1999), "Friday");
}

#[test]
fn test_day_of_the_week9() {
    assert_eq!(Solution::day_of_the_week(1, 1, 2001), "Monday");
}

#[test]
fn test_day_of_the_week10() {
    assert_eq!(Solution::day_of_the_week(1, 1, 2000), "Saturday");
}

#[test]
fn test_day_of_the_week11() {
    assert_eq!(Solution::day_of_the_week(1, 3, 1900), "Monday");
}

#[test]
fn test_day_of_the_week12() {
    assert_eq!(Solution::day_of_the_week(15, 6, 2024), "Saturday");
}

#[test]
fn test_day_of_the_week13() {
    assert_eq!(Solution::day_of_the_week(30, 11, 1985), "Saturday");
}

#[test]
fn test_day_of_the_week14() {
    assert_eq!(Solution::day_of_the_week(20, 4, 1975), "Sunday");
}

#[test]
fn test_day_of_the_week15() {
    assert_eq!(Solution::day_of_the_week(5, 1, 1971), "Tuesday");
}

#[test]
fn test_day_of_the_week16() {
    assert_eq!(Solution::day_of_the_week(6, 1, 1971), "Wednesday");
}
