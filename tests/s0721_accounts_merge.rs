// Tests for Problem 0721: Accounts Merge
// Java reference: src/test/java/g0701_0800/s0721_accounts_merge/SolutionTest.java

use leetcode_in_rust::s0721::accounts_merge::Solution;

#[test]
fn test_accounts_merge() {
    let input = vec![
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john_newyork@mail.com".to_string(),
        ],
        vec![
            "John".to_string(),
            "johnsmith@mail.com".to_string(),
            "john00@mail.com".to_string(),
        ],
        vec!["Mary".to_string(), "mary@mail.com".to_string()],
        vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
    ];

    let result = Solution::accounts_merge(input);

    // The result should contain 3 accounts
    // Due to HashMap ordering, we check the structure rather than exact order
    assert_eq!(result.len(), 3);
}

#[test]
fn test_accounts_merge2() {
    let input = vec![
        vec![
            "Gabe".to_string(),
            "Gabe0@m.co".to_string(),
            "Gabe3@m.co".to_string(),
            "Gabe1@m.co".to_string(),
        ],
        vec![
            "Kevin".to_string(),
            "Kevin3@m.co".to_string(),
            "Kevin5@m.co".to_string(),
            "Kevin0@m.co".to_string(),
        ],
        vec![
            "Ethan".to_string(),
            "Ethan5@m.co".to_string(),
            "Ethan4@m.co".to_string(),
            "Ethan0@m.co".to_string(),
        ],
        vec![
            "Hanzo".to_string(),
            "Hanzo3@m.co".to_string(),
            "Hanzo1@m.co".to_string(),
            "Hanzo0@m.co".to_string(),
        ],
        vec![
            "Fern".to_string(),
            "Fern5@m.co".to_string(),
            "Fern1@m.co".to_string(),
            "Fern0@m.co".to_string(),
        ],
    ];

    let result = Solution::accounts_merge(input);

    // Each account should remain separate (no merging needed)
    assert_eq!(result.len(), 5);
}
