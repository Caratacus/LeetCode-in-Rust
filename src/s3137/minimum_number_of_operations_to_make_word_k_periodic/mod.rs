// Problem 3137: minimum number of operations to make word k periodic
// #Medium #String #Hash_Table #Greedy
// #2024_05_07_Time_7 ms(100.00%)  Space 42.9 MB (65.82%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let k = k as usize;
        let n = word.len();
        let mut max_freq = 0;

        // For each position modulo k, count character frequencies
        for i in 0..k {
            let mut freq: HashMap<char, i32> = HashMap::new();
            let mut j = i;
            while j < n {
                let ch = word.chars().nth(j).unwrap();
                *freq.entry(ch).or_insert(0) += 1;
                j += k;
            }
            // Find max frequency for this position
            let max = freq.values().max().copied().unwrap_or(0);
            max_freq += max;
        }

        (n - max_freq as usize) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_operations_to_make_k_periodic() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic(String::from("leetcode"), 4),
            0
        );
    }

    #[test]
    fn test_minimum_operations_to_make_k_periodic2() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic(String::from("leetcoleet"), 2),
            0
        );
    }
}
