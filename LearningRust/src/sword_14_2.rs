//python3
//class Solution:
//     def cuttingRope(self, n: int) -> int:
//         if n <= 3: return n - 1
//         a, b, p = n // 3, n % 3, 1000000007
//         if b == 0: return 3 ** a % p
//         if b == 1: return 3 ** (a - 1) * 4 % p
//         return 3 ** a * 2 % p
impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n < 4 { return n - 1; }
        let (mut a, mut rem, mut x) = (n / 3 as i32 - 1, 1_i64, 3_i64);
        while a > 0 {
            if a % 2 == 1 {
                rem = (rem * x) % 1000000007;
            }
            x = x * x % 1000000007;
            a /= 2;
        }
        match n % 3 {
            0 => (rem * 3 % 1000000007) as i32,
            1 => (rem * 4 % 1000000007) as i32,
            _ => (rem * 6 % 1000000007) as i32
        }
    }
}