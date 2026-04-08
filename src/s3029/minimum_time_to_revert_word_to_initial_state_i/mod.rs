// Problem 3029: Minimum Time to Revert Word to Initial State I
// #Medium #String #Hash_Function #String_Matching #Rolling_Hash

pub struct Solution;

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let n = word.len();
        let word_bytes = word.as_bytes();

        let mut i = k as usize;
        while i < n {
            // Check if word[i..n] equals word[0..n-i]
            if word_bytes[i..n] == word_bytes[0..n - i] {
                return (i / k as usize) as i32;
            }
            i += k as usize;
        }

        ((n + k as usize - 1) / k as usize) as i32
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
