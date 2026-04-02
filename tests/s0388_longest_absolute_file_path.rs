// Tests for Problem 0388: Longest Absolute File Path
// Java reference: src/test/java/g0301_0400/s0388_longest_absolute_file_path/SolutionTest.java

use leetcode_in_rust::s0388::longest_absolute_file_path::Solution;

#[test]
fn test_length_longest_path() {
    let input = "dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext";
    assert_eq!(Solution::length_longest_path(input.to_string()), 20);
}

#[test]
fn test_length_longest_path2() {
    let input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
    assert_eq!(Solution::length_longest_path(input.to_string()), 32);
}

#[test]
fn test_length_longest_path3() {
    let input = "a";
    assert_eq!(Solution::length_longest_path(input.to_string()), 0);
}

#[test]
fn test_length_longest_path4() {
    let input = "file1.txt\nfile2.txt\nlongfile.txt";
    assert_eq!(Solution::length_longest_path(input.to_string()), 12);
}
