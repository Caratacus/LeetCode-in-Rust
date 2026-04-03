// Tests for Problem 0811: Subdomain Visit Count
// Java reference: src/test/java/g0701_0800/s0811_subdomain_visit_count/SolutionTest.java

use leetcode_in_rust::s0811::subdomain_visit_count::Solution;

#[test]
fn test_subdomain_visits() {
    let mut result = Solution::subdomain_visits(vec![
        "9001 discuss.leetcode.com".to_string()
    ]);
    result.sort();
    assert_eq!(result, vec!["9001 com", "9001 leetcode.com", "9001 discuss.leetcode.com"]);
}

