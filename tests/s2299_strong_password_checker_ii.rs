// Tests for Problem 2299: Strong Password Checker II
// Java reference: src/test/java/g2201_2300/s2299_strong_password_checker_ii/SolutionTest.java

use leetcode_in_rust::s2299::strong_password_checker_ii::Solution;

#[test]
fn test_strong_password_checker_ii() {
    assert_eq!(Solution::strong_password_checker_ii(String::from("IloveLe3tcode!")), true);
}

#[test]
fn test_strong_password_checker_ii2() {
    assert_eq!(Solution::strong_password_checker_ii(String::from("Me+You--IsMyDream")), false);
}

#[test]
fn test_strong_password_checker_ii3() {
    assert_eq!(Solution::strong_password_checker_ii(String::from("1aB!")), false);
}

#[test]
fn test_strong_password_checker_ii4() {
    assert_eq!(
        Solution::strong_password_checker_ii(String::from(
            "ecuwcfoyajkolntovfniplayrxhzpmhrkhzonopcwxgupzhoupw"
        )),
        false
    );
}

#[test]
fn test_strong_password_checker_ii5() {
    assert_eq!(Solution::strong_password_checker_ii(String::from("\"|{}\"|{}")), false);
}
