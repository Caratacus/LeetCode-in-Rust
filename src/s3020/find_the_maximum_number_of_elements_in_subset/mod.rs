// Problem 3020: Find the Maximum Number of Elements in Subset
// #Medium #Array #Hash_Table #Enumeration

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        Self::with_hash_map(&nums)
    }

    fn with_hash_map(nums: &[i32]) -> i32 {
        let mut map: HashMap<i64, i32> = HashMap::new();
        for &i in nums {
            *map.entry(i as i64).or_insert(0) += 1;
        }

        let mut ans = 0;
        if let Some(&count) = map.get(&1) {
            if count % 2 == 0 {
                ans = count - 1;
            } else {
                ans = count;
            }
        }

        for &key in map.keys() {
            if key == 1 {
                continue;
            }
            let len = Self::find_series(&map, key);
            ans = ans.max(len);
        }

        ans
    }

    fn find_series(map: &HashMap<i64, i32>, key: i64) -> i32 {
        let sqr = key * key;
        if let Some(&count) = map.get(&sqr) {
            if let Some(&key_count) = map.get(&key) {
                if key_count >= 2 {
                    return 2 + Self::find_series(map, sqr);
                }
            }
            1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void maximumLength()
    //   assertThat(new Solution().maximumLength(new int[] {5, 4, 1, 2, 2}), equalTo(3));
    #[test]
    fn test_maximum_length() {
        assert_eq!(Solution::maximum_length(vec![5, 4, 1, 2, 2]), 3);
    }

    // Java: void maximumLength2()
    //   assertThat(new Solution().maximumLength(new int[] {1, 3, 2, 4}), equalTo(1));
    #[test]
    fn test_maximum_length2() {
        assert_eq!(Solution::maximum_length(vec![1, 3, 2, 4]), 1);
    }
}
