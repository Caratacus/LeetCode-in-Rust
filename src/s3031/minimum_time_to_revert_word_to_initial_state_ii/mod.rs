// Problem 3031: minimum time to revert word to initial state ii
// #Hard #String #Hash_Function #String_Matching #Rolling_Hash
// #2024_03_01_Time_24_ms_(74.98%)_Space_55.1_MB_(14.85%)

pub struct Solution;

impl Solution {
    pub fn minimum_time_to_initial_state(w: String, q: i32) -> i32 {
        let c: Vec<char> = w.chars().collect();
        let len = c.len();
        let mut lps = vec![0; len];

        for i in 1..len {
            if c[i] == c[0] {
                lps[i] = 1;
            }
            let mut k = lps[i - 1];
            while k > 0 {
                if c[k] == c[i] {
                    lps[i] = k + 1;
                    break;
                }
                k = lps[k - 1];
            }
        }

        let mut k = lps[len - 1];
        while k > 0 {
            if (len - k) % q as usize == 0 {
                return ((len - k) / q as usize) as i32;
            }
            k = lps[k - 1];
        }

        ((len + q as usize - 1) / q as usize) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minimumTimeToInitialState()
    //   assertThat(new Solution().minimumTimeToInitialState("abacaba", 3), equalTo(2));
    #[test]
    fn test_minimum_time_to_initial_state() {
        assert_eq!(Solution::minimum_time_to_initial_state("abacaba".to_string(), 3), 2);
    }

    // Java: void minimumTimeToInitialState2()
    //   assertThat(new Solution().minimumTimeToInitialState("abacaba", 4), equalTo(1));
    #[test]
    fn test_minimum_time_to_initial_state2() {
        assert_eq!(Solution::minimum_time_to_initial_state("abacaba".to_string(), 4), 1);
    }

    // Java: void minimumTimeToInitialState3()
    //   assertThat(new Solution().minimumTimeToInitialState("abcbabcd", 2), equalTo(4));
    #[test]
    fn test_minimum_time_to_initial_state3() {
        assert_eq!(Solution::minimum_time_to_initial_state("abcbabcd".to_string(), 2), 4);
    }
}
