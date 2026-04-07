// Tests for Problem 2125: Number of Laser Beams in a Bank
// Java reference: src/test/java/g2101_2200/s2125_number_of_laser_beams_in_a_bank/SolutionTest.java

use leetcode_in_rust::s2125::number_of_laser_beams_in_a_bank::Solution;

#[test]
fn test_number_of_beams() {
    assert_eq!(
        Solution::number_of_beams(vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string()
        ]),
        8
    );
}

#[test]
fn test_number_of_beams2() {
    assert_eq!(
        Solution::number_of_beams(vec!["000".to_string(), "111".to_string(), "000".to_string()]),
        0
    );
}
