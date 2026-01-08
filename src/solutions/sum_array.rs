pub fn run(input: Option<Vec<i32>>) {
    let input_array = match input {
        Some(array) => array,
        None => [1, 2, 3, 4, 10, 11].into(),
    };

    println!("Input {:?}", &input_array);
    println!("Output: {}", sum_array(&input_array))
}

/// Calculates the sum of all elements in an array of integers.
///
/// # Arguments
///
/// * `array` - A slice of integers to sum
///
/// # Returns
///
/// The sum of all elements in the array as an `i32`
///
/// # Examples
///
/// Basic array summation:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr = [1, 2, 3, 4, 10, 11];
/// assert_eq!(sum_array(&arr), 31);
/// // 1 + 2 + 3 + 4 + 10 + 11 = 31
/// ```
///
/// Array with negative numbers:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr = [5, -3, 2, -1];
/// assert_eq!(sum_array(&arr), 3);
/// // 5 + (-3) + 2 + (-1) = 3
/// ```
///
/// Single element array:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr = [42];
/// assert_eq!(sum_array(&arr), 42);
/// ```
///
/// Empty array:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr: [i32; 0] = [];
/// assert_eq!(sum_array(&arr), 0);
/// // Sum of empty array is 0
/// ```
///
/// Array with all zeros:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr = [0, 0, 0, 0];
/// assert_eq!(sum_array(&arr), 0);
/// ```
///
/// Large array:
///
/// ```
/// # use solutions::simple_array_sum::sum_array;
/// let arr = [100, 200, 300, 400, 500];
/// assert_eq!(sum_array(&arr), 1500);
/// ```
///
/// # Constraints
///
/// The sum must be within the valid range of `i32` (-2,147,483,648 to 2,147,483,647).
fn sum_array(array: &[i32]) -> i32 {
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
