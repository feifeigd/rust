/*!
 * This crate provides functionality for adding things
 * 
 * # Example
 * ```
 * use xtnet::sum;
 * 
 * let work_a = 4;
 * let work_b = 34;
 * let total_work = sum(work_a, work_b);
 * ```
 */


/// cute
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// pow
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

/// Sum tow arguments
/// 
/// # Example
/// 
/// ```
/// assert_eq!(xtnet::sum(1, 1), 2);
/// ```
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

pub fn slow_fibonacci(nth: usize)->u64{
    if nth <=1 {
        return nth as u64;
    }
    return slow_fibonacci(nth -1) + slow_fibonacci(nth -2);
}

pub fn fast_fibonacci(nth:usize)->u64{
    let mut a=0;
    let mut b = 1;
    let mut c =0;
    for _ in 1..nth {
        c=a+b;
        a=b;
        b=c;
    }
    c
}