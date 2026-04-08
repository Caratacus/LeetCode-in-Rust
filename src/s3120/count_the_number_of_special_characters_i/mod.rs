// Problem 3120: count the number of special characters i
// #Easy #String #Hash_Table #2024_04_27_Time_1_ms(100.00%)  Space_41.9 MB (92.08%)

pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut a = [0; 26];
        let mut b = [0; 26];
        let mut ans = 0;

        for c in word.chars() {
            if c >= 'a' && c <= 'z' {
                a[(c as usize) - ('a' as usize)] += 1;
            } else {
                b[(c as usize) - ('A' as usize)] += 1;
            }
        }

        for i in 0..26 {
            if a[i] != 0 && b[i] != 0 {
                ans += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_special_chars() {
        assert_eq!(Solution::number_of_special_chars(String::from("aaAbcBC")), 3);
    }

    #[test]
    fn test_number_of_special_chars2() {
        assert_eq!(Solution::number_of_special_chars(String::from("abc")), 0);
    }

    #[test]
    fn test_number_of_special_chars3() {
        assert_eq!(Solution::number_of_special_chars(String::from("abBCab")), 1);
    }
}
