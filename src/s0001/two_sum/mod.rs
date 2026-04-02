// Problem 0001: two sum
// #Easy #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table
// #Data_Structure_I_Day_2_Array #Level_1_Day_13_Hashmap #Udemy_Arrays #Top_Interview_150_Hashmap
// #Big_O_Time_O(n)_Space_O(n)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in numbers.iter().enumerate() {
            let required_num = target - num;
            if let Some(&prev_index) = index_map.get(&required_num) {
                return vec![prev_index, i as i32];
            }
            index_map.insert(num, i as i32);
        }
        vec![-1, -1]
    }
}

