// Tests for Problem 1169: Invalid Transactions
// Java reference: src/test/java/g1101_1200/s1169_invalid_transactions/SolutionTest.java

use leetcode_in_rust::s1169::invalid_transactions::Solution;

#[test]
fn test_invalid_transactions() {
    let mut result = Solution::invalid_transactions(vec![
        "alice,20,800,mtv".to_string(),
        "alice,50,100,beijing".to_string(),
    ]);
    result.sort();
    assert_eq!(
        result,
        vec!["alice,20,800,mtv".to_string(), "alice,50,100,beijing".to_string()]
    );
}

#[test]
fn test_invalid_transactions2() {
    assert_eq!(
        Solution::invalid_transactions(vec![
            "alice,20,800,mtv".to_string(),
            "alice,50,1200,mtv".to_string()
        ]),
        vec!["alice,50,1200,mtv".to_string()]
    );
}

#[test]
fn test_invalid_transactions3() {
    assert_eq!(
        Solution::invalid_transactions(vec![
            "alice,20,800,mtv".to_string(),
            "bob,50,1200,mtv".to_string()
        ]),
        vec!["bob,50,1200,mtv".to_string()]
    );
}
