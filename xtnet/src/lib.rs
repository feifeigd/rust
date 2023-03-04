pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn pow(base: i64, exponent: usize) -> i64 {
    let mut res = 1;
    for _ in 0..exponent {
        res *= base;
    }
    res
}

pub fn greet() {
    println!("Hi from xtnet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn minus_two_raised_three_is_minus_eight() {
        assert_eq!(pow(-2, 0), 1);
        assert_eq!(pow(-2, 3), -8);
    }
}
