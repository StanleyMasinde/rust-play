pub fn run(input: i32) {
    let output = extra_long_factorial(&input);

    println!("Input: {}", input);
    println!("Output: {}", output)
}

/// The factorial of the integer *n*, written *n!*, is defined as:
///  ```text
///    n! = n x (n - 1) x (n - 2) x ...x 3 x 2 x 1
///  ```
/// Calculate and print the factorial of a given integer.
/// For example, if *n = 30* , we calculate *30 x 29 * x ... x 2 x 1* and get *265252859812191058636308480000000*.
/// # Function Description
/// Complete the *extra_long_factorials* function in the editor below. It should print the result and return.
/// *extra_long_factorials* has the following parameter(s):
/// - n: an integer
/// *Note:* Factorials of *n > 20* can't be stored even in a  long long variable. Big integers must be used for such calculations. Languages like Java, Python, Ruby etc. can handle big integers, but we need to write additional code in C/C++ to handle huge values.
/// We recommend solving this challenge using BigIntegers.
/// # Input Format
/// Input consists of a single integer
/// # Constraints
/// ```text
///  1 <= n <= 100
/// ```
/// # Output Format
/// Print the factorial of *n*.
/// # Sample Input
/// 25
/// # Sample Output
/// ```text
/// 15511210043330985984000000
/// ```
/// # Explanation
/// ```text
/// 25! = 25 x 24 x 23 ...x 3 x 2 x 1
/// ```
fn extra_long_factorial(n: &i32) -> String {
    let mut digits = vec![1u8];
    let upper_limit = n.clone();

    for k in 2..=upper_limit {
        let mut carry = 0;
        for d in &mut digits {
            let prod = *d as i32 * k + carry;
            *d = (prod % 10) as u8;
            carry = prod / 10;
        }
        while carry > 0 {
            digits.push((carry % 10) as u8);
            carry /= 10;
        }
    }

    format!(
        "{}",
        digits
            .iter()
            .rev()
            .map(|d| char::from(b'0' + d))
            .collect::<String>()
    )
}

#[cfg(test)]
mod test {
    use crate::solutions::extra_long_factorial::extra_long_factorial;

    #[test]
    fn test_extra_long_factorial() {
        assert_eq!(extra_long_factorial(&5), "120".to_string());
    }
}
