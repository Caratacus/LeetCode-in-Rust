// Problem 3121: count the number of special characters ii
// #Medium #String #Hash_Table #2024_04_27_Time_6_ms_(100.00%)_Space_45.2_MB_(97.93%)

pub struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut small: [i32; 26] = [-1; 26];
        let mut capital: [i32; 26] = [i32::MAX; 26];
        let mut result = 0;
        for (i, a) in word.chars().enumerate() {
            let idx = i as i32;
            if (a as u8) < 91 {
                let pos = (a as usize) - 65;
                capital[pos] = capital[pos].min(idx);
            } else {
                let pos = (a as usize) - 97;
                small[pos] = idx;
            }
        }
        for i in 0..26 {
            if small[i] != -1 && capital[i] != i32::MAX && capital[i] > small[i] {
                result += 1;
            }
        }
        result
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
        assert_eq!(Solution::number_of_special_chars(String::from("AbBCab")), 0);
    }
}
