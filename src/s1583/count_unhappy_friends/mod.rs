// Problem 1583: Count Unhappy Friends
// #Medium #Array #Simulation
// #Big_O_Time_O(n^2)_Space_O(n)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let mut unhappy_friends = 0;
        let mut assigned_pair: HashMap<i32, i32> = HashMap::new();

        for pair in &pairs {
            assigned_pair.insert(pair[0], pair[1]);
            assigned_pair.insert(pair[1], pair[0]);
        }

        for pair in &pairs {
            if Self::is_unhappy(pair[1], pair[0], &preferences, &assigned_pair) {
                unhappy_friends += 1;
            }
            if Self::is_unhappy(pair[0], pair[1], &preferences, &assigned_pair) {
                unhappy_friends += 1;
            }
        }
        unhappy_friends
    }

    fn is_unhappy(
        myself: i32,
        assigned_friend: i32,
        preferences: &[Vec<i32>],
        assigned_pairs: &HashMap<i32, i32>,
    ) -> bool {
        let preference = &preferences[myself as usize];
        let assigned_friend_preference_index = Self::find_index(preference, assigned_friend);

        for i in 0..=assigned_friend_preference_index {
            let preferred_friend = preference[i];
            let preferred_friend_assigned_friend = *assigned_pairs.get(&preferred_friend).unwrap();

            if preferred_friend_assigned_friend == myself {
                return false;
            }

            let candidate_assigned_friend_index = Self::find_index(
                &preferences[preferred_friend as usize],
                preferred_friend_assigned_friend,
            );

            if Self::is_preferred(
                myself,
                &preferences[preferred_friend as usize],
                candidate_assigned_friend_index,
            ) {
                return true;
            }
        }
        false
    }

    fn is_preferred(myself: i32, preference: &[i32], boundary: usize) -> bool {
        for i in 0..=boundary {
            if myself == preference[i] {
                return true;
            }
        }
        false
    }

    fn find_index(preference: &[i32], assigned_friend: i32) -> usize {
        for (i, &val) in preference.iter().enumerate() {
            if val == assigned_friend {
                return i;
            }
        }
        0
    }
}
