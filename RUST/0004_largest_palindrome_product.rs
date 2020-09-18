fn is_palindrome(num: i32) -> bool {
    let mut text: Vec<_> = num.to_string().chars().collect();

    while text.len() > 1 {
        if text[0] != text.pop().unwrap() {
            return false;
        } else {
            text.remove(0);
        }
    }
    true
}

fn main() {
    let sample: Vec<_> = (100..999).collect();
    let mut vec_prod: Vec<i32> = vec![];
    let mut i = 999;

    while i != 1 {
        let vec_local: Vec<i32> = sample.iter().rev()
                              .map(|x| x * i)
                              .filter(|&x| is_palindrome(x))
                              .collect();

        vec_prod.extend(vec_local);

        i-=1;
    }

    println!("{:?}", vec_prod.iter().max());
}