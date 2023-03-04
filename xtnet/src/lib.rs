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

pub fn sum(a: i8, b: i8) -> i8 {
    a + b
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

    fn sum_inputs_outputs() -> Vec<((i8, i8), i8)> {
        vec![((1, 1), 2), ((0, 0), 0), ((2, -2), 0)]
    }
    #[test]
    fn test_sums() {
        for (input, output) in sum_inputs_outputs() {
            assert_eq!(sum(input.0, input.1), output);
        }
    }
}
