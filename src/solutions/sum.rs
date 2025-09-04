pub fn run(a: Option<i32>, b: Option<i32>) {
    println!("{}", sum(a.unwrap_or(10), b.unwrap_or(20)));
}

/// Complete the `sum` function to compute the sum of two integers.
///
/// # Function Description
/// The function takes two integers and returns their sum.
///
/// ## Parameters
/// - `a`: the first integer
/// - `b`: the second integer
///
/// ## Returns
/// - An integer representing the sum of `a` and `b`
///
/// ## Example
/// ```
/// Input: a = 2, b = 3
/// Output: 5
/// Explanation: 2 + 3 = 5
/// ```
///
/// ## Constraints
/// - The input integers are within the valid range of `i32`.
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
