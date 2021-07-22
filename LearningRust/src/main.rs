fn main() {
    let i = 120;
    let x = cutting_rope(i);
    print!("{}", x)
}

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
        0 => (rem * 3) as i32,
        1 => (rem * 4) as i32,
        _ => (rem * 6) as i32
    }
}