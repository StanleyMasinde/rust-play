pub fn run(input: Option<Vec<i32>>) {
    let input_array = match input {
        Some(array) => array,
        None => [1, 2, 3, 4, 10, 11].into(),
    };

    println!("Input {:?}", &input_array);
    println!("Output: {}", sum_array(&input_array))
}

/// Given an array of integers, find the sum of its elements.
///
/// # Function Description
/// Complete the `sum_array` function with the following parameter:
/// - `ar`: an array of integers
///
/// # Returns
/// - The sum of the array elements
///
/// # Input Format
/// - The first line contains an integer `n`, the size of the array.
/// - The second line contains `n` space-separated integers, the elements of the array.
///
/// # Constraints
/// - Assumes valid `i32` inputs; no explicit bounds given
///
/// # Sample Input
/// ```text
/// 6
/// 1 2 3 4 10 11
/// ```
///
/// # Sample Output
/// ```text
/// 31
/// ```
///
/// # Explanation
/// The sum of the array elements is 1 + 2 + 3 + 4 + 10 + 11 = 31.
pub fn sum_array(array: &[i32]) -> i32 {
    array.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_array() {
        assert_eq!(sum_array(&[1, 2, 3, 4, 10, 11]), 31);
    }
}
