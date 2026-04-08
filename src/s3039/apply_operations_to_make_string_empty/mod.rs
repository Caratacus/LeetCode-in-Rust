// Problem 3039: apply operations to make string empty
// #Medium #Array #Hash_Table #Sorting #Counting
// #2024_03_04_Time_18_ms_(93.00%)_Space_48.3_MB_(94.05%)

pub struct Solution;

impl Solution {
    pub fn last_non_empty_string(s: String) -> String {
        let mut freq = vec![0; 26];
        let ar: Vec<char> = s.chars().collect();
        let n = ar.len();
        let mut max = 1;
        let mut result = String::new();

        for c in &ar {
            freq[(*c as usize) - ('a' as usize)] += 1;
            max = max.max(freq[(*c as usize) - ('a' as usize)]);
        }

        for i in (0..n).rev() {
            if freq[(ar[i] as usize) - ('a' as usize)] == max {
                result.push(ar[i]);
                freq[(ar[i] as usize) - ('a' as usize)] = 0;
            }
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void lastNonEmptyString()
    //   assertThat(new Solution().lastNonEmptyString("aabcbbca"), equalTo("ba"));
    #[test]
    fn test_last_non_empty_string() {
        assert_eq!(Solution::last_non_empty_string("aabcbbca".to_string()), "ba");
    }

    // Java: void lastNonEmptyString2()
    //   assertThat(new Solution().lastNonEmptyString("abcd"), equalTo("abcd"));
    #[test]
    fn test_last_non_empty_string2() {
        assert_eq!(Solution::last_non_empty_string("abcd".to_string()), "abcd");
    }
}
