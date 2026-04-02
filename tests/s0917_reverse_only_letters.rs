// Tests for Problem 0917: Reverse Only Letters
// Java reference: src/test/java/g0901_1000/s0917_reverse_only_letters/SolutionTest.java

use leetcode_in_rust::s0917::reverse_only_letters::Solution;

#[test]
fn test_reverse_only_letters() {
    assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
}

#[test]
fn test_reverse_only_letters2() {
    assert_eq!(Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()), "j-Ih-gfE-dCba");
}

#[test]
fn test_reverse_only_letters3() {
    assert_eq!(
        Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
        "Qedo1ct-eeLg=ntse-T!"
    );
}
