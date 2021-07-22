//python3
//class Solution:
//     def cuttingRope(self, n: int) -> int:
//         if n < 4:
//             return n - 1
//         mod = n % 3
//         num = n // 3
//         if mod == 0:
//             return int(math.pow(3, num))
//         if mod == 1:
//             return int(math.pow(3, (num - 1)) * 4)
//         return int(math.pow(3, num) * 2)
impl Solution {
    pub fn cutting_rope(n: i32) -> i32 {
        if n < 4 {
            return n - 1;
        }
        match n % 3 {
            0 => 3_i32.pow(n.div_euclid(3) as u32),
            1 => 3_i32.pow((n.div_euclid(3) - 1) as u32) * 4,
            _ => 3_i32.pow(n.div_euclid(3) as u32) * 2;
        }
    }
}