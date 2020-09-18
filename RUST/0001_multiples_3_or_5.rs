fn main() {
    let n: i32 = 1000;
    let is_mul_3_5 = |x| { x % 3 == 0 || x % 5 == 0 };

    let sum = (0..n).filter(|&x| is_mul_3_5(x))
                    .fold(0, |acc, x| acc + x); 


    assert_eq!(sum, 233168);
}
