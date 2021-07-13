
pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n
        }
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        for i in 2..=n {
            let sum = (a + b) % 1000000007;
            a = b;
            b = sum;
        }
        b
    }
//python3
//class Solution:
//    def fib(self, n: int) -> int:
//        a, b = 0, 1
//        for _ in range(n):
//            a, b = b, a + b
//        return a % 1000000007
