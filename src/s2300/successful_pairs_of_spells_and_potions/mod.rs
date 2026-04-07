// Problem 2300: successful pairs of spells and potions

pub struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort();
        let m = potions.len();

        spells
            .iter()
            .map(|&spell| {
                let min_potion = (success + spell as i64 - 1) / spell as i64;
                // Find first position where element >= min_potion
                let idx = potions.partition_point(|x| (*x as i64) < min_potion);
                (m - idx) as i32
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void successfulPairs()
    //   assertThat(
    //   new Solution().successfulPairs(new int[] {5, 1, 3}, new int[] {1, 2, 3, 4, 5}, 7),
    //   equalTo(new int[] {4, 0, 3}));
    #[test]
    fn test_successful_pairs() {
        assert_eq!(
            Solution::successful_pairs(vec
![5, 1, 3], vec
![1, 2, 3, 4, 5], 7),
            vec
![4, 0, 3]
        );
    }

    // Java: void successfulPairs2()
    //   assertThat(
    //   new Solution().successfulPairs(new int[] {3, 1, 2}, new int[] {8, 5, 8}, 16),
    //   equalTo(new int[] {2, 0, 2}));
    #[test]
    fn test_successful_pairs2() {
        assert_eq!(
            Solution::successful_pairs(vec
![3, 1, 2], vec
![8, 5, 8], 16),
            vec
![2, 0, 2]
        );
    }
}
