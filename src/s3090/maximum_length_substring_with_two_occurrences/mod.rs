// Problem 3090: maximum length substring with two occurrences
// #Easy #String #Hash_Table #Sliding_Window #2024_04_18_Time_1_ms_(100.00%)_Space_42.5_MB_(55.33%)

pub struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut freq = [0i32; 26];
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        let len = s.len();
        let mut max = 0;

        for j in 0..len {
            freq[(chars[j] as usize) - ('a' as usize)] += 1;
            while freq[(chars[j] as usize) - ('a' as usize)] == 3 {
                freq[(chars[i] as usize) - ('a' as usize)] -= 1;
                i += 1;
            }
            max = std::cmp::max(max, (j - i + 1) as i32);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumLengthSubstring()
    //   assertThat(new Solution().maximumLengthSubstring("bcbbbcba"), equalTo(4));
    #[test]
    fn test_maximum_length_substring() {
        assert_eq!(Solution::maximum_length_substring("bcbbbcba".to_string()), 4);
    }

    // Java: void maximumLengthSubstring2()
    //   assertThat(new Solution().maximumLengthSubstring("aaaa"), equalTo(2));
    #[test]
    fn test_maximum_length_substring2() {
        assert_eq!(Solution::maximum_length_substring("aaaa".to_string()), 2);
    }
}
