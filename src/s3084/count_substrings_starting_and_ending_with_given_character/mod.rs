// Problem 3084: count substrings starting and ending with given character
// #Medium #String #Math #Counting

pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String, c: char) -> i64 {
        let count = s.chars().filter(|&ch| ch == c).count() as i64;
        count * (count + 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countSubstrings()
    //   assertThat(new Solution().countSubstrings("abada", 'a'), equalTo(6L));
    #[test]
    fn test_count_substrings() {
        assert_eq!(Solution::count_substrings("abada".to_string(), 'a'), 6);
    }

    // Java: void countSubstrings2()
    //   assertThat(new Solution().countSubstrings("zzz", 'z'), equalTo(6L));
    #[test]
    fn test_count_substrings2() {
        assert_eq!(Solution::count_substrings("zzz".to_string(), 'z'), 6);
    }
}
