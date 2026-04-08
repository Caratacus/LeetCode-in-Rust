// Problem 3046: Split the Array
// #Easy #Array #Hash_Table #Counting
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut count = [0i32; 101];
        for &n in &nums {
            let idx = n as usize;
            if idx < 101 {
                count[idx] += 1;
                if count[idx] > 2 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_possible_to_split() {
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]), true);
    }

    #[test]
    fn test_is_possible_to_split2() {
        assert_eq!(Solution::is_possible_to_split(vec![1, 1, 1, 1]), false);
    }
}
