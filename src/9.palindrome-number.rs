/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut n = x.abs();
        let mut reverse = 0;

        while n != 0 {
            // get last digit using n % 10
            // append to reverse
            reverse = reverse * 10 + (n % 10);

            // remove last digit using n / 10
            n /= 10;
        }

        return x == reverse;
    }
}
// @lc code=end

