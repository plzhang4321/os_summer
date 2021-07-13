fn main() {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let mut a: i32 = 0;
        let mut b: i32 = 1;
        for _ in 2..=n {
            let sum = (a + b) % 1000000007;
            a = b;
            b = sum;
        }
        b
    }
    let x = fib(4);
    print!("{}", x)
}
