pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    (1..=n).product()
}

fn main() {
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(3) = {}", is_even(3));
    println!("factorial(0) = {}", factorial(0));
    println!("factorial(1) = {}", factorial(1));
    println!("factorial(5) = {}", factorial(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        // 偶数
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        // 奇数
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_factorial() {
        // 0!
        assert_eq!(factorial(0), 1);
        // 1!
        assert_eq!(factorial(1), 1);
        // 5!
        assert_eq!(factorial(5), 120);
    }
}
