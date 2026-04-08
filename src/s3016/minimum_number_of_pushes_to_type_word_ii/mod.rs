// Problem 3016: minimum number of pushes to type word ii
// #Medium #String #Hash_Table #Sorting #Greedy #Breadth_First_Search #Graph #Prefix_Sum #Counting

pub struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut count = [0i32; 26];
        for ch in word.chars() {
            count[(ch as u8 - b'a') as usize] += 1;
        }

        let mut j = 8;
        let mut result = 0;

        loop {
            let mut mi = 0;
            for i in 0..26 {
                if count[mi] < count[i] {
                    mi = i;
                }
            }
            if count[mi] == 0 {
                break;
            }
            result += (j / 8) * count[mi];
            count[mi] = 0;
            j += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_pushes() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
    }

    #[test]
    fn test_minimum_pushes2() {
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
    }

    #[test]
    fn test_minimum_pushes3() {
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()),
            24
        );
    }
}
