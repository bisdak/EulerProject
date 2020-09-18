fn main() {
    let mut a = 1;
    let mut b = 2;
    let limit: i32 = 4_000_000;
    let mut sum = 2;

    loop {
        let c = a + b;
        if c > limit { break; };

        if c % 2 == 0 {
            sum += c;
        }

        a = b;
        b = c;
    };

    assert_eq!(sum, 4613732);
}
