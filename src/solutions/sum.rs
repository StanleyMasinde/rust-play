pub fn run(a: Option<i32>, b: Option<i32>) {
    println!("{}", sum(a.unwrap_or(10), b.unwrap_or(20)));
}

/// Computes the sum of two integers.
///
/// # Arguments
///
/// * `a` - The first integer
/// * `b` - The second integer
///
/// # Returns
///
/// An integer representing the sum of `a` and `b`
///
/// # Examples
///
/// Basic addition of positive numbers:
///
/// ```
/// # use solutions::simple_array_sum::sum;
/// assert_eq!(sum(2, 3), 5);
/// // 2 + 3 = 5
/// ```
///
/// Adding negative numbers:
///
/// ```
/// # use solutions::simple_array_sum::sum;
/// assert_eq!(sum(-5, -10), -15);
/// // -5 + (-10) = -15
/// ```
///
/// Adding positive and negative numbers:
///
/// ```
/// # use solutions::simple_array_sum::sum;
/// assert_eq!(sum(10, -3), 7);
/// // 10 + (-3) = 7
/// ```
///
/// Adding zero:
///
/// ```
/// # use solutions::simple_array_sum::sum;
/// assert_eq!(sum(42, 0), 42);
/// // 42 + 0 = 42
/// ```
///
/// # Constraints
///
/// The input integers must be within the valid range of `i32` (-2,147,483,648 to 2,147,483,647).
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(2, 3), 5);
        assert_eq!(sum(100, 1000), 1100)
    }
}
