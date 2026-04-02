// Tests for Problem 1154: Day of the Year
// Java reference: src/test/java/g1101_1200/s1154_day_of_the_year/SolutionTest.java

use leetcode_in_rust::s1154::day_of_the_year::Solution;

#[test]
fn test_day_of_year() {
    assert_eq!(Solution::day_of_year("2019-01-09".to_string()), 9);
}

#[test]
fn test_day_of_year2() {
    assert_eq!(Solution::day_of_year("2019-02-10".to_string()), 41);
}

#[test]
fn test_day_of_year3() {
    assert_eq!(Solution::day_of_year("2020-02-01".to_string()), 32);
}

#[test]
fn test_day_of_year4() {
    assert_eq!(Solution::day_of_year("2020-03-01".to_string()), 61);
}

#[test]
fn test_day_of_year5() {
    assert_eq!(Solution::day_of_year("2019-12-31".to_string()), 365);
}

#[test]
fn test_day_of_year6() {
    assert_eq!(Solution::day_of_year("2020-12-31".to_string()), 366);
}
