/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // penulisan romawi besar -> kecil, kiri -> kanan
        // anomali ketika ada romawi kecil di kiri besasr

        let symbols: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        
        let mut result: i32 = *symbols.get(&s.chars().last().unwrap()).unwrap();
        
        let mut is_add = true;
        for i in (1..s.len()).rev() {
            let left_val = symbols.get(&s.chars().nth(i-1).unwrap()).unwrap();
            let right_val = symbols.get(&s.chars().nth(i).unwrap()).unwrap();

            if left_val > right_val {
                is_add = true;
            } else if left_val < right_val {
                is_add = false;
            }

            if is_add {
                result += left_val;
            } else {
                result -= left_val;
            }
        }

        result

    }
}
// @lc code=end

