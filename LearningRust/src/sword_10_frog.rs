impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        let (mut a, mut b) = (1, 1);
        for _ in 2..=n {
            let temp = (a + b) % 1000000007;
            a = b;
            b = temp;
        }
        b
    }
}

//python3
//class Solution:
//     def numWays(self, n: int) -> int:
//         a, b = 1, 1
//         for _ in range(n):
//             a, b = b, a + b
//         return a % 1000000007
//