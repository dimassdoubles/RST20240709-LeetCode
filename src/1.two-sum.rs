/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = i32::try_from(nums.len()).unwrap();

        let mut map: HashMap<i32, i32> = HashMap::new();    
        map.insert(nums[0], 0);

        for i in 1..length {
            let pair: i32 = target - nums[i as usize];
            let result = map.get(&pair);

            match result {
                Some(x) => {
                    return vec![*x, i];
                }
                _ => {}
            }
            
            map.insert(nums[i as usize], i);
        }

        let result: Vec<i32> = Vec::new();
        return result;
    }
}
// @lc code=end

