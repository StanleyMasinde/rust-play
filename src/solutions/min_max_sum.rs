pub(crate) fn run(input: &[i64]) {
    let result = mini_max_sum(input);
    println!("Input: {:?}", input);
    println!("Output: {:?}", result);
}

/// Calculates and prints the minimum and maximum sums of exactly four integers from an array of five.
///
/// Given an array of five positive integers, this function computes:
/// - The minimum sum: obtained by summing the four smallest values (excluding the maximum)
/// - The maximum sum: obtained by summing the four largest values (excluding the minimum)
///
/// The results are printed as two space-separated integers on a single line.
///
/// # Arguments
///
/// * `arr` - A slice containing exactly 5 positive integers
///
/// # Constraints
///
/// * `1 <= arr[i] <= 10^9`
/// * Array must contain exactly 5 elements
///
/// # Integer Overflow
///
/// This function uses 64-bit integers (`i64`) to prevent overflow when summing values,
/// as the sum of four integers each up to 10^9 can exceed 32-bit integer limits.
///
/// # Examples
///
/// Basic example with small numbers:
///
/// ```
/// # fn mini_max_sum(arr: &[i32]) {
/// #     let total: i64 = arr.iter().map(|&x| x as i64).sum();
/// #     let min_val = *arr.iter().min().unwrap() as i64;
/// #     let max_val = *arr.iter().max().unwrap() as i64;
/// #     println!("{} {}", total - max_val, total - min_val);
/// # }
/// let arr = [1, 2, 3, 4, 5];
/// mini_max_sum(&arr);
/// // Output: 10 14
/// // Explanation:
/// // - Minimum sum: 1+2+3+4 = 10 (excluding 5)
/// // - Maximum sum: 2+3+4+5 = 14 (excluding 1)
/// ```
///
/// Example with larger values:
///
/// ```
/// # fn mini_max_sum(arr: &[i32]) {
/// #     let total: i64 = arr.iter().map(|&x| x as i64).sum();
/// #     let min_val = *arr.iter().min().unwrap() as i64;
/// #     let max_val = *arr.iter().max().unwrap() as i64;
/// #     println!("{} {}", total - max_val, total - min_val);
/// # }
/// let arr = [1, 3, 5, 7, 9];
/// mini_max_sum(&arr);
/// // Output: 16 24
/// // Explanation:
/// // - Minimum sum: 1+3+5+7 = 16 (excluding 9)
/// // - Maximum sum: 3+5+7+9 = 24 (excluding 1)
/// ```
///
/// Example demonstrating overflow prevention with large values:
///
/// ```
/// # fn mini_max_sum(arr: &[i32]) {
/// #     let total: i64 = arr.iter().map(|&x| x as i64).sum();
/// #     let min_val = *arr.iter().min().unwrap() as i64;
/// #     let max_val = *arr.iter().max().unwrap() as i64;
/// #     println!("{} {}", total - max_val, total - min_val);
/// # }
/// let arr = [256741038, 623958417, 467905213, 714532089, 938071625];
/// mini_max_sum(&arr);
/// // Output: 2063136757 2744467344
/// ```
fn mini_max_sum(arr: &[i64]) -> (i64, i64) {
    let mut all_sums: Vec<i64> = Vec::with_capacity(5);
    let total_array_sum = arr.iter().sum::<i64>();

    for i in 0..5 {
        let value_to_remove = arr[i];
        let current_total = total_array_sum - value_to_remove;
        all_sums.push(current_total);
    }
    let minimum = all_sums.iter().min().unwrap().to_owned();
    let maximum = all_sums.iter().max().unwrap().to_owned();

    (minimum, maximum)
}

#[cfg(test)]
mod test {
    use crate::solutions::min_max_sum::mini_max_sum;

    #[test]
    fn test_min_max() {
        let input = [1, 3, 5, 7, 9];
        assert_eq!(mini_max_sum(&input), (16, 24))
    }

    #[test]
    fn test_sample_input() {
        let input = [1, 2, 3, 4, 5];
        assert_eq!(mini_max_sum(&input), (10, 14))
    }

    #[test]
    fn test_large_values() {
        let input = [256741038, 623958417, 467905213, 714532089, 938071625];
        assert_eq!(mini_max_sum(&input), (2063136757, 2744467344))
    }

    #[test]
    fn test_all_same_values() {
        let input = [5, 5, 5, 5, 5];
        assert_eq!(mini_max_sum(&input), (20, 20))
    }

    #[test]
    fn test_max_constraint_values() {
        let input = [1000000000, 1000000000, 1000000000, 1000000000, 1000000000];
        assert_eq!(mini_max_sum(&input), (4000000000, 4000000000))
    }

    #[test]
    fn test_min_constraint_values() {
        let input = [1, 1, 1, 1, 1];
        assert_eq!(mini_max_sum(&input), (4, 4))
    }

    // #[test]
    // fn test_mixed_small_large() {
    //     let input = [1, 1, 1, 1000000000, 1000000000];
    //    assert_eq!(mini_max_sum(&input), (1000000003, 2000000003))
    // }

    // #[test]
    // fn test_two_extremes() {
    //   let input = [1, 2, 3, 999999999, 1000000000];
    //  assert_eq!(mini_max_sum(&input), (6, 2000000004))
    // }
}
